from pathlib import Path

from django.core.files.base import ContentFile
from django.core.management.base import BaseCommand

from hawk_processing import process_report
from ...models import Report


class Command(BaseCommand):
    help = "Re-processes all reports from their raw version."

    def handle(self, *args, **options):
        for report in Report.objects.all():
            raw_report = report.raw_report.read().decode("utf-8")

            try:
                processed = process_report(raw_report)
            except ValueError as e:
                self.stderr.write(self.style.WARNING(f"Unable to reprocess report {report.slug} ({report.uuid} - "
                                                     f"{report.title}) - it may be outdated?"))
                self.stderr.write(str(e))
                continue
            except RuntimeError as e:
                self.stderr.write(self.style.ERROR(f"Error while reprocessing report {report.slug}"))
                self.stderr.write(str(e))
                continue

            # We first remove the file to avoid Django creating another with a
            # suffix—we don't care if we overwrite the old one
            processed_filepath = Path(report.processed_report.path)
            if processed_filepath.exists():
                processed_filepath.unlink()

            report.processed_report.save(
                "report", ContentFile(processed["processed_report"]), save=False
            )

            report.uuid = processed["match_uuid"]
            report.title = processed["title"]
            report.minecraft_version = processed.get("minecraft_version", None)

            report.generator_name = processed.get("generator_name", None)
            report.generator_link = processed.get("generator_link", None)

            report.save()

            self.stdout.write(
                f"Re-processed report {report.slug} ({report.uuid} - {report.title})"
            )

        self.stdout.write(self.style.SUCCESS("Done."))
