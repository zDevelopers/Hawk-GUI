from django.urls import path

from .views import ReportView, MinecraftHeadView

# fmt: off
urlpatterns = [
    path("head/<uuid:uuid>/<int:size>", MinecraftHeadView.as_view(), name="minecraft-head"),
    path("<slug>", ReportView.as_view(), name="report")
]
