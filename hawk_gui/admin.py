from django.contrib import admin
from django.db.models import F
from django.utils.translation import gettext_lazy as _

from .models import Report


class MinecraftVersionFilter(admin.SimpleListFilter):
    parameter_name = "minecraft_version"
    title = _("Minecraft version")

    def lookups(self, request, model_admin):
        choices = []

        for version in (
            Report.objects.distinct()
            .order_by(F("minecraft_version").desc(nulls_last=True))
            .values_list("minecraft_version", flat=True)
        ):
            if version:
                choices.append(
                    (
                        version,
                        version
                        if not version or version.lower().startswith("minecraft")
                        else f"Minecraft {version}",
                    )
                )
            else:
                choices.append(("__unknown__", _("Unknown version")))

        return choices

    def queryset(self, request, queryset):
        if self.value() == "__unknown__":
            return queryset.filter(minecraft_version__isnull=True)
        elif self.value() is not None:
            return queryset.filter(minecraft_version=self.value())


class GeneratorFilter(admin.SimpleListFilter):
    parameter_name = "generator"
    title = _("generator")

    def lookups(self, request, model_admin):
        choices = []

        for generator in (
            Report.objects.distinct()
            .order_by(F("generator_name").desc(nulls_last=True))
            .values_list("generator_name", flat=True)
        ):
            if generator:
                choices.append((generator, generator))
            else:
                choices.append(("__unknown__", _("Unspecified generator")))

        return choices

    def queryset(self, request, queryset):
        if self.value() == "__unknown__":
            return queryset.filter(generator_name__isnull=True)
        elif self.value() is not None:
            return queryset.filter(generator_name=self.value())


@admin.register(Report)
class ReportAdmin(admin.ModelAdmin):
    date_hierarchy = "published_at"

    list_display = (
        "get_slug_for_admin",
        "get_uuid_for_admin",
        "published_at",
        "get_minecraft_version_full",
        "get_generator_with_html_link",
        "views_count",
    )

    list_filter = (MinecraftVersionFilter, GeneratorFilter)

    fields = (
        "slug",
        "uuid",
        "raw_report",
        "processed_report",
        "published_at",
        "published_by",
        "minecraft_version",
        "generator_name",
        "generator_link",
        "views_count",
    )
    readonly_fields = (
        "uuid",
        "published_at",
        "published_by",
        "minecraft_version",
        "generator_name",
        "generator_link",
        "views_count",
        "processed_report",
    )
