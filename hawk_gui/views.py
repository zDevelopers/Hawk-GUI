import json

from pathlib import Path

import requests
from django.utils.decorators import method_decorator
from django.views.decorators.csrf import csrf_exempt
from ipware import get_client_ip

from django.conf import settings
from django.core.files.base import ContentFile
from django.db.models import F
from django.http import FileResponse, Http404, JsonResponse, HttpRequest
from django.urls import reverse
from django.utils.functional import cached_property
from django.views.generic import View, DetailView
from django.views.generic.detail import SingleObjectMixin

from hawk_processing import process_report

from . import __version__ as hawk_version
from .models import Report


class ReportView(DetailView):
    model = Report
    template_name = "report.html"
    context_object_name = "db_report"

    def get_context_data(self, **kwargs):
        Report.objects.filter(pk=self.object.pk).update(
            views_count=F("views_count") + 1
        )

        report = json.load(self.object.processed_report)

        context = super().get_context_data(**kwargs)
        context["report"] = report

        if report["settings"]["summary"]["enabled"]:
            context["default_tab"] = "summary"
        elif report["settings"]["damages"]["enabled"]:
            context["default_tab"] = "damages"
        else:
            context["default_tab"] = "players"

        return context


class ReportJSONView(View, SingleObjectMixin):
    model = Report

    def get(self, request, **kwargs):
        report = json.load(self.get_object().processed_report)
        del report["match_uuid"]
        return JsonResponse(report)


@method_decorator(csrf_exempt, name="dispatch")
class PublicationView(View):
    def post(self, request: HttpRequest):
        try:
            raw_report = request.body.decode("utf-8").strip()
            processed_report = process_report(raw_report)
            ip, _routable = get_client_ip(request)

            # TODO check previously-uploaded reports from this IP for abuse

            # Check if there is a report with the same UUID
            # TODO Check for key in headers (?), update if valid, and send update signal if live
            if Report.objects.filter(uuid=processed_report["match_uuid"]).exists():
                return JsonResponse(
                    {
                        "error": "Trying to update an existing report without key",
                        "error_code": "hawk::UPDATE_WITHOUT_KEY",
                        "description": "There is a report with the same UUID but your request does not include the "
                        "update secret key. (Note: reports update is not yet supported so don't look for a key "
                        "anywhere yet.)",
                    },
                    status=403,
                )

            report = Report(
                uuid=processed_report["match_uuid"],
                published_by=ip,
                minecraft_version=processed_report.get("minecraft_version"),
                generator_name=processed_report.get("generator_name"),
                generator_link=processed_report.get("generator_link"),
            )

            report.raw_report.save("raw_report", ContentFile(raw_report), save=False)
            report.processed_report.save(
                "report", ContentFile(processed_report["processed_report"]), save=False
            )

            report.save()

            return JsonResponse(
                {
                    "uri": request.build_absolute_uri(
                        reverse("report", args=(report.slug,))
                    )
                },
                status=201,
            )

        except UnicodeDecodeError as e:
            return JsonResponse(
                {
                    "error": "Unable to decode JSON report",
                    "error_code": "hawk::JSON_DECODE",
                    "description": f"Unable to decode JSON report: {e}",
                },
                status=400,
            )

        except RuntimeError as e:
            return JsonResponse(
                {
                    "error": "Internal error while processing report",
                    "error_code": "hawk::INTERNAL_PROCESSING_ERROR",
                    "description": f"Internal error while processing JSON: {e}",
                },
                status=500,
            )

        except ValueError as e:
            return JsonResponse(
                {
                    "error": "Invalid report",
                    "error_code": "hawk::INVALID_REPORT",
                    "description": str(e),
                },
                status=400,
            )

    def http_method_not_allowed(self, request, *args, **kwargs):
        response = super().http_method_not_allowed(request, *args, **kwargs)
        response.content = json.dumps(
            {
                "error": "This endpoint can only be used with HTTP POST requests.",
                "error_code": "hawk::METHOD_NOT_ALLOWED",
                "description": "POST to this URL a JSON file representing a match to get an online report page.",
            }
        )
        response["Content-Type"] = "application/json"
        return response


class MinecraftHeadView(View):
    @cached_property
    def session(self):
        session = requests.Session()
        session.headers.update(
            {
                "User-Agent": f"Mozilla/5.0 (compatible; Hawk-Heads-Fetcher/{hawk_version}; "
                f"+https://amaury.carrade.eu/contact)"
            }
        )
        return session

    def get(self, request, uuid, size):
        filename = (
            Path(settings.MEDIA_ROOT) / "heads" / str(uuid)[:2] / f"{uuid}-{size}.png"
        )

        if not filename.exists():
            r = self.session.get(
                f"https://crafatar.com/avatars/{uuid}?overlay&size={size}"
            )
            if not r.ok:
                raise Http404

            filename.parent.mkdir(parents=True, exist_ok=True)

            with filename.open("wb") as f:
                for chunk in r.iter_content(chunk_size=128):
                    f.write(chunk)

        return FileResponse(filename.open("rb"))
