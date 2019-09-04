import math
from datetime import datetime, timedelta

from django import template
from django.utils.safestring import mark_safe
from django.utils.translation import gettext_lazy as _, ngettext_lazy


register = template.Library()


@register.filter
def duration(duration):
    """
    Displays a duration.

    :param duration: The duration. Either a timedelta, a
                     dict {"secs": xx, "nanos": xx} or an
                     int (number of seconds).
    :return: The formatted duration (dd) hh:mm:ss.
    """
    total_seconds = None
    if isinstance(duration, timedelta):
        total_seconds = duration.total_seconds()
    elif isinstance(duration, dict) and "secs" in duration:
        total_seconds = duration["secs"]
    else:
        try:
            total_seconds = int(duration)
        except ValueError:
            total_seconds = None

    if total_seconds is None:
        return _("00:00")

    days = total_seconds // 86400
    hours = (total_seconds - days * 86400) // 3600
    minutes = (total_seconds - (days * 86400) - (hours * 3600)) // 60
    seconds = total_seconds - (days * 86400) - (hours * 3600) - (minutes * 60)

    if days != 0:
        return _("%dd %02d:%02d:%02d") % (days, hours, minutes, seconds)
    elif hours != 0:
        return _("%02d:%02d:%02d") % (hours, minutes, seconds)
    else:
        return _("%02d:%02d") % (minutes, seconds)


@register.filter
def iso_to_datetime(iso_date):
    try:
        return datetime.fromisoformat(iso_date)
    except ValueError:
        return iso_date


@register.inclusion_tag("partials/player.html")
def player(player, large=False, visible_color=True):
    return {
        "player": player,
        "large": large,
        "size": 48 if large else 32,
        "colored": visible_color,
    }


@register.inclusion_tag("partials/hearts.html")
def hearts(hearts_count, lethal=False, list=False):
    if isinstance(hearts_count, dict):
        lethal = hearts_count.get("lethal", False)
        hearts_count = hearts_count.get("damage", hearts_count.get("heal", 0))
    else:
        hearts_count = int(hearts_count)

    # Computes the amount of hearts on each line
    lines = [20] * math.ceil(hearts_count / 20) if hearts_count != 0 else [0]
    if hearts_count % 20 != 0:
        lines[-1] = hearts_count % 20

    return {"lines": lines, "lethal": lethal, "display_in_list": list}


@register.inclusion_tag("partials/tooltips/player_title.html")
def player_tooltip_title(player):
    return {"player": player}


@register.inclusion_tag("partials/tooltips/player.html")
def player_tooltip(player):
    return {"player": player}


@register.inclusion_tag("partials/tooltips/damage.html")
def damage_tooltip(damage):
    hearts_count = damage["damage"] // 2

    if damage["damage"] % 2 == 0:
        hearts_count_for_display = ngettext_lazy(
            "%d heart against %s", "%d hearts against %s", number=hearts_count
        ) % (damage["damage"] // 2, damage["damagee"]["name"])
    elif damage["damage"] == 1:
        hearts_count_for_display = _("Half a heart against %s") % (
            damage["damagee"]["name"]
        )
    else:
        hearts_count_for_display = ngettext_lazy(
            "%d heart and a half against %s",
            "%d hearts and a half against %s",
            number=hearts_count,
        ) % (damage["damage"] // 2, damage["damagee"]["name"])

    return {
        "damage": damage,
        "hearts_count": hearts_count,
        "hearts_count_for_display": hearts_count_for_display,
    }


@register.inclusion_tag("partials/tooltips/weapon.html")
def weapon_tooltip(weapon, weapon_name=None, weapon_enchantments=None):
    return {
        "weapon": weapon,
        "weapon_name": weapon_name,
        "weapon_enchantments": weapon_enchantments,
    }


@register.inclusion_tag("partials/tooltips/heal.html")
def heal_tooltip(heal):
    hearts_count = heal["heal"] // 2

    if heal["heal"] % 2 == 0:
        hearts_count_for_display = ngettext_lazy(
            "%d heart regenerated", "%d hearts regenerated", number=hearts_count
        ) % (heal["heal"] // 2)
    elif heal["heal"] == 1:
        hearts_count_for_display = _("Half a heart regenerated")
    else:
        hearts_count_for_display = ngettext_lazy(
            "%d heart and a half regenerated",
            "%d hearts and a half regenerated",
            number=hearts_count,
        ) % (heal["heal"] // 2)

    return {
        "heal": heal,
        "hearts_count": hearts_count,
        "hearts_count_for_display": hearts_count_for_display,
    }
