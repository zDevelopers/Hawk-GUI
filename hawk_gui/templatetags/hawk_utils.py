from django import template

register = template.Library()


@register.filter
def lookup_key(dict, key):
    try:
        return dict[key]
    except KeyError:
        return ""
