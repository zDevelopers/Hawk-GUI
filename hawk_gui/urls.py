from django.urls import path

from .views import ReportView, ReportJSONView, PublicationView, MinecraftHeadView

# fmt: off
urlpatterns = [
    path("head/<uuid:uuid>/<int:size>", MinecraftHeadView.as_view(), name="minecraft-head"),

    path("publish", PublicationView.as_view(), name="publish"),
    path("<slug>", ReportView.as_view(), name="report"),
    path("<slug>/as-json", ReportJSONView.as_view(), name="report-json")
]
