from datetime import datetime, timedelta

import math
from functools import reduce

from django import template
from django.utils.text import format_lazy
from django.utils.translation import gettext_lazy as _, ngettext_lazy

register = template.Library()


def deep_get(dictionary, path, default=None):
    """
    Finds the value represented by the path in the dictionary, or the default
    value if not found.

    :param dictionary: The dictionary.
    :param path: The path in the dictionary (e.g. “key.sub-key.sub-sub-key”).
    :param default: The default value returned if not found.
    :return: The value found, or the default.
    """
    return reduce(lambda d, key: d.get(key, default) if isinstance(d, dict) else default, path.split("."), dictionary)


@register.filter
def duration(duration, long=False):
    """
    Displays a duration.

    :param duration: The duration. Either a timedelta, a
                     dict {"secs": xx, "nanos": xx} or an
                     int (number of seconds).
    :param long: True to display the duration in long format
    :return: The formatted duration (dd) hh:mm:ss or “(dd days), hh hours mm minutes and ss seconds”.
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

    if long:
        if days != 0:
            return format_lazy(
                _("{days:d} days, {hours:d} hours and {minutes:d} minutes"),
                days=days,
                hours=hours,
                minutes=minutes,
            )
        elif hours != 0:
            return format_lazy(
                _("{hours:d} hours and {minutes:d} minutes"),
                hours=hours,
                minutes=minutes,
            )
        elif minutes != 0:
            return format_lazy(
                _("{minutes:d} minutes and {seconds:d} seconds"),
                minutes=minutes,
                seconds=seconds,
            )
        else:
            return format_lazy(_("{seconds:d} seconds"), seconds=seconds)
    else:
        if days != 0:
            return format_lazy(
                _("{days:d}d {hours:02d}:{minutes:02d}:{seconds:02d}"),
                days=days,
                hours=hours,
                minutes=minutes,
                seconds=seconds,
            )
        elif hours != 0:
            return format_lazy(
                _("{hours:02d}:{minutes:02d}:{seconds:02d}"),
                hours=hours,
                minutes=minutes,
                seconds=seconds,
            )
        else:
            return format_lazy(
                _("{minutes:02d}:{seconds:02d}"), minutes=minutes, seconds=seconds
            )


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
        hearts_count_for_display = format_lazy(
            ngettext_lazy(
                "{damage} heart against {damagee}",
                "{damage} hearts against {damagee}",
                number=hearts_count,
            ),
            damage=damage["damage"] // 2,
            damagee=damage["damagee"]["name"],
        )
    elif damage["damage"] == 1:
        hearts_count_for_display = _("Half a heart against %s") % (
            damage["damagee"]["name"]
        )
    else:
        hearts_count_for_display = format_lazy(
            ngettext_lazy(
                "{damage} heart and a half against {damagee}",
                "{damage} hearts and a half against {damagee}",
                number=hearts_count,
            ),
            damage=damage["damage"] // 2,
            damagee=damage["damagee"]["name"],
        )

    return {
        "damage": damage,
        "hearts_count": hearts_count,
        "hearts_count_for_display": hearts_count_for_display,
    }


@register.inclusion_tag("partials/tooltips/item.html")
def item_tooltip(item):
    return {
        "item": item,
        "item_name": deep_get(item, "tag.display.Name", ""),
        "item_enchantments": (deep_get(item, 'tag.Enchantments', []) or []) + (deep_get(item, 'tag.StoredEnchantments', []) or []),
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
        hearts_count_for_display = format_lazy(
            ngettext_lazy(
                "{heal} heart and a half regenerated",
                "{heal} hearts and a half regenerated",
                number=hearts_count,
            ),
            heal=heal["heal"] // 2,
        )

    return {
        "heal": heal,
        "hearts_count": hearts_count,
        "hearts_count_for_display": hearts_count_for_display,
    }
