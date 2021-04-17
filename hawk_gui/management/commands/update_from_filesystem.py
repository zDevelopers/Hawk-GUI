import json
from datetime import datetime
from pathlib import Path
from uuid import UUID

from django.conf import settings
from django.core.management.base import BaseCommand
from glob import glob

from hawk_processing import strip_minecraft_color_codes
from ...models import Report


class Command(BaseCommand):
    help = "Imports existing reports into the database, or updates them if they already exist."

    def handle(self, *args, **options):
        for report_dir in glob(str(settings.MEDIA_ROOT) + "/reports/**/*/"):
            report_dir = Path(report_dir)
            report_filename = report_dir / "report.json"
            report_slug = report_dir.name

            if not report_filename.exists() or not report_filename.is_file():
                self.stderr.write(
                    self.style.WARNING(
                        f"Directory {report_dir} does not contain any report, skipping."
                    )
                )
                continue

            # Filenames relative to MEDIA_ROOT (used for db storage)
            relative_raw_report_filename = str(report_dir / "raw-report.json").replace(
                str(settings.MEDIA_ROOT) + "/", ""
            )
            relative_report_filename = str(report_filename).replace(
                str(settings.MEDIA_ROOT) + "/", ""
            )

            with report_filename.open() as report_file:
                report = json.load(report_file)

                report_uuid = UUID(report["match_uuid"])
                report_title = strip_minecraft_color_codes(report["title"])
                report_minecraft = (
                    report["minecraft"] if "minecraft" in report else None
                )

                report_generator = None
                report_generator_link = None

                if (
                    "settings" in report
                    and report["settings"]
                    and "generator" in report["settings"]
                ):
                    report_generator = (
                        report["settings"]["generator"]["name"]
                        if report["settings"]["generator"]
                        and "name" in report["settings"]["generator"]
                        else None
                    )
                    report_generator_link = (
                        report["settings"]["generator"]["link"]
                        if report["settings"]["generator"]
                        and "link" in report["settings"]["generator"]
                        else None
                    )

                try:
                    db_report: Report = Report.objects.get(slug=report_slug)

                    db_report.uuid = report_uuid
                    db_report.title = report_title
                    db_report.minecraft_version = report_minecraft

                    db_report.generator_name = report_generator
                    db_report.generator_link = report_generator_link

                    db_report.raw_report.name = relative_raw_report_filename
                    db_report.processed_report.name = relative_report_filename

                    db_report.save()

                    self.stdout.write(
                        f"Updated existing report {report_slug} ({report_uuid})."
                    )

                except Report.DoesNotExist:
                    db_report = Report(
                        slug=report_slug,
                        uuid=report_uuid,
                        title=report_title,
                        published_by=None,  # no info available
                        minecraft_version=report_minecraft,
                        generator_name=report_generator,
                        generator_link=report_generator_link,
                    )

                    db_report.raw_report.name = relative_raw_report_filename
                    db_report.processed_report.name = relative_report_filename

                    db_report.save()

                    # Better than nothing. We update it after because else `auto_now_add`
                    # overwrites the value.
                    db_report.published_at = datetime.fromisoformat(report["date"])
                    db_report.save()

                    self.stdout.write(
                        self.style.SUCCESS(
                            f"Inserted new report {report_slug} ({report_uuid})"
                        )
                    )
