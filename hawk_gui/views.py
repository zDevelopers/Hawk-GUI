import json

from pathlib import Path

import requests

from django.conf import settings
from django.db.models import F
from django.http import FileResponse, Http404
from django.utils.functional import cached_property
from django.views.generic import View, DetailView

from . import __version__ as hawk_version
from .models import Report


class ReportView(DetailView):
    model = Report
    template_name = "report.html"

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
