from django.conf import settings # import the settings file


def inject_hawk_settings(request):
    return {'hawk_settings': settings.HAWK}
