from uuid import UUID

from django import template
from django.urls.base import reverse_lazy
from django.utils.safestring import mark_safe
from django.utils.translation import gettext_lazy as _

from hawk_processing import (
    parse_minecraft_color_codes,
    strip_minecraft_color_codes,
    to_roman,
)

register = template.Library()


@register.filter
def minecraft(value):
    """
    Converts a Minecraft formatted string to HTML.
    :param value: The Minecraft formatted string.
    :return: The HTML conversion.
    """
    return mark_safe(parse_minecraft_color_codes(value))


@register.filter
def strip_minecraft(value):
    """
    Converts a Minecraft formatted string to HTML.
    :param value: The Minecraft formatted string.
    :return: The HTML conversion.
    """
    return mark_safe(strip_minecraft_color_codes(value))


@register.filter
def head(player, size=16):
    """
    From an UUID or a player dict containing an `uuid` key, returns the
    URL to its head.

    :param player: The player or UUID.
    :param size: The requested size (default 16 px).
    :return: The head's URL.
    """
    try:
        uuid = UUID(player["uuid"] if "uuid" in player else player)
    except ValueError:
        print(f'Invalid UUID given: {player["uuid"] if "uuid" in player else player}')
        uuid = UUID("00000000-0000-0000-0000-000000000000")

    return reverse_lazy("minecraft-head", args=(uuid, size))


@register.filter
def color_to_css(color):
    """
    Converts a color name into its corresponding CSS class.

    :param color: The color (e.g. DARK_GREEN).
    :return: The CSS class part (e.g. dark-green).
    """
    return mark_safe(str(color).lower().replace("_", "-").replace(" ", "-"))


ENCHANTMENTS = {
    "aqua_affinity": _("Aqua Affinity"),
    "bane_of_arthropods": _("Bane of Arthropods"),
    "blast_protection": _("Blast Protection"),
    "channeling": _("Channeling"),
    "binding_curse": _("Curse of Binding"),
    "vanishing_curse": _("Curse of Vanishing"),
    "depth_strider": _("Depth Strider"),
    "efficiency": _("Efficiency"),
    "feather_falling": _("Feather Falling"),
    "fire_aspect": _("Fire Aspect"),
    "fire_protection": _("Fire Protection"),
    "flame": _("Flame"),
    "fortune": _("Fortune"),
    "frost_walker": _("Frost Walker"),
    "impaling": _("Impaling"),
    "infinity": _("Infinity"),
    "knockback": _("Knockback"),
    "looting": _("Looting"),
    "loyalty": _("Loyalty"),
    "luck_of_the_sea": _("Luck of the Sea"),
    "lure": _("Lure"),
    "mending": _("Mending"),
    "multishot": _("Multishot"),
    "piercing": _("Piercing"),
    "power": _("Power"),
    "projectile_protection": _("Projectile Protection"),
    "protection": _("Protection"),
    "punch": _("Punch"),
    "quick_charge": _("Quick Charge"),
    "respiration": _("Respiration"),
    "riptide": _("Riptide"),
    "sharpness": _("Sharpness"),
    "silk_touch": _("Silk Touch"),
    "smite": _("Smite"),
    "sweeping": _("Sweeping Edge"),
    "thorns": _("Thorns"),
    "unbreaking": _("Unbreaking"),
}

LEVEL_1_ONLY_ENCHANTMENTS = [
    "aqua_affinity",
    "silk_touch",
    "flame",
    "infinity",
    "mending",
    "binding_curse",
    "vanishing_curse",
    "channeling",
    "multishot",
]

CURSED_ENCHANTMENTS = ["binding_curse", "vanishing_curse"]


@register.filter
def enchantment(enchant: str, level=1):
    """
    From a Minecraft enchantment raw name and a level,
    outputs a formatted Minecraft enchantment name like
    in the game.

    As example,
    "depth_strider" | enchant:2 == "Depth Strider II"
    "silk_touch" | enchant:1 == "Silk Touch" (level-1-only enchantment)
    "solidity" | enchant:1 == "Solidity I"

    Internationalisation is supported.

    :param enchant: The Minecraft vanilla enchantment name (with
                    or without namespace)
    :param level: The enchantment level (default to 1)
    :return: The formatted enchantment
    """
    enchant = enchant.replace("minecraft:", "").lower()
    enchant_name = ENCHANTMENTS[enchant] if enchant in ENCHANTMENTS else enchant

    if enchant in LEVEL_1_ONLY_ENCHANTMENTS and level == 1:
        enchant_level = ""
    else:
        enchant_level = f" {to_roman(level)}"

    formatted_enchant = f"{enchant_name}{enchant_level}"

    if enchant in CURSED_ENCHANTMENTS:
        formatted_enchant = parse_minecraft_color_codes(f"§c{formatted_enchant}")

    return mark_safe(formatted_enchant)


NAMES = {
    # Damage causes
    "PLAYER": _("Player"),
    "ZOMBIE": _("Zombie"),
    "SKELETON": _("Skeleton"),
    "PIGMAN": _("Pigman"),
    "WITCH": _("Witch"),
    "SPIDER": _("Spider"),
    "CAVE_SPIDER": _("Cave Spider"),
    "CREEPER": _("Creeper"),
    "ENDERMAN": _("Enderman"),
    "SLIME": _("Slime"),
    "GHAST": _("Ghast"),
    "MAGMA_CUBE": _("Magme Cube"),
    "BLAZE": _("Blaze"),
    "WOLF": _("Wolf"),
    "ANGRY_WOLF": _("Angry Wolf"),
    "SILVERFISH": "Silverfish",
    "IRON_GOLEM": _("Iron Golem"),
    "ZOMBIE_VILLAGER": _("Zombie Villager"),
    "ENDER_DRAGON": _("Ender Dragon"),
    "WITHER": _("Wither Boss"),
    "WITHER_SKELETON": _("Wither Skeleton"),
    "FIRE": _("Fire"),
    "LAVA": _("Lava"),
    "THUNDERBOLT": _("Thunder"),
    "CACTUS": _("Cactus"),
    "TNT": _("TNT"),
    "FALL": _("Fall"),
    "SUFFOCATION": _("Suffocation"),
    "DROWNING": _("Drowning"),
    "STARVATION": _("Starvation"),
    "COMMAND": _("Command"),
    # Weapons
    "FISTS": _("Fists"),
    "SWORD_WOOD": _("Wooden Sword"),
    "SWORD_STONE": _("Stone Sword"),
    "SWORD_IRON": _("Iron Sword"),
    "SWORD_GOLD": _("Golden Sword"),
    "SWORD_DIAMOND": _("Diamond Sword"),
    "AXE_WOOD": _("Wooden Axe"),
    "AXE_STONE": _("Stone Axe"),
    "AXE_IRON": _("Iron Axe"),
    "AXE_GOLD": _("Golden Axe"),
    "AXE_DIAMOND": _("Diamond Axe"),
    "PICKAXE_WOOD": _("Wooden Pickaxe"),
    "PICKAXE_STONE": _("Stone Pickaxe"),
    "PICKAXE_IRON": _("Iron Pickaxe"),
    "PICKAXE_GOLD": _("Golden Pickaxe"),
    "PICKAXE_DIAMOND": _("Diamond Pickaxe"),
    "HOE_WOOD": _("Wooden Hoe"),
    "HOE_STONE": _("Stone Hoe"),
    "HOE_IRON": _("Iron Hoe"),
    "HOE_GOLD": _("Golden Hoe"),
    "HOE_DIAMOND": _("Diamond Hoe"),
    "SHOVEL_WOOD": _("Wooden Shovel"),
    "SHOVEL_STONE": _("Stone Shovel"),
    "SHOVEL_IRON": _("Iron Shovel"),
    "SHOVEL_GOLD": _("Golden Shovel"),
    "SHOVEL_DIAMOND": _("Diamond Shovel"),
    "BOW": _("Bow"),
    "MAGIC": _("Magic"),
    "THORNS": _("Thorns"),
    # Healing causes
    "NATURAL": _("Natural Regeneration"),
    "GOLDEN_APPLE": _("Golden Apple"),
    "NOTCH_APPLE": _("Enchanted Golden Apple"),
    "HEALING_POTION": _("Healing Potion"),
    # Others
    "UNKNOWN": _("Unknown"),
}

ICONS = {
    # Damage causes
    "PLAYER": "",
    "ZOMBIE": "entity-zombie-small",
    "PIGMAN": "entity-zombie-pigman-small",
    "FIRE": "block-fire-small",
    "LAVA": "block-lava-small",
    "THUNDERBOLT": "entity-lightning-small",
    "CACTUS": "block-cactus-small",
    "TNT": "block-tnt-small",
    "FALL": "block-stone-small",
    "SUFFOCATION": "block-sand-small",
    "DROWNING": "block-water-small",
    "STARVATION": "item-rotten-flesh-small",
    "COMMAND": "entity-command-block-small",
    # Weapons
    "FISTS": "",
    "SWORD_WOOD": "item-wood-sword-small",
    "SWORD_STONE": "item-stone-sword-small",
    "SWORD_IRON": "item-iron-sword-small",
    "SWORD_GOLD": "item-gold-sword-small",
    "SWORD_DIAMOND": "item-diamond-sword-small",
    "AXE_WOOD": "item-wood-axe-small",
    "AXE_STONE": "item-stone-axe-small",
    "AXE_IRON": "item-iron-axe-small",
    "AXE_GOLD": "item-gold-axe-small",
    "AXE_DIAMOND": "item-diamond-axe-small",
    "PICKAXE_WOOD": "item-wood-pickaxe-small",
    "PICKAXE_STONE": "item-stone-pickaxe-small",
    "PICKAXE_IRON": "item-iron-pickaxe-small",
    "PICKAXE_GOLD": "item-gold-pickaxe-small",
    "PICKAXE_DIAMOND": "item-diamond-pickaxe-small",
    "HOE_WOOD": "item-wood-hoe-small",
    "HOE_STONE": "item-stone-hoe-small",
    "HOE_IRON": "item-iron-hoe-small",
    "HOE_GOLD": "item-gold-hoe-small",
    "HOE_DIAMOND": "item-diamond-hoe-small",
    "SHOVEL_WOOD": "item-wood-shovel-small",
    "SHOVEL_STONE": "item-stone-shovel-small",
    "SHOVEL_IRON": "item-iron-shovel-small",
    "SHOVEL_GOLD": "item-gold-shovel-small",
    "SHOVEL_DIAMOND": "item-diamond-shovel-small",
    "BOW": "item-bow-pulling-small",
    "MAGIC": "item-potion-bottle-splash-small",
    "THORNS": "item-diamond-chestplate-small",
    # Healing causes
    "NATURAL": "item-potato-baked-small",
    "GOLDEN_APPLE": "item-apple-golden-small",
    "NOTCH_APPLE": "item-apple-golden-small",
    "HEALING_POTION": "item-potion-bottle-splash-small",
    # Others
    "UNKNOWN": "entity-unknown-small",
}

for thing in NAMES.keys():
    if thing not in ICONS:
        ICONS[thing] = f"entity-{thing.lower().replace('_', '-')}-small"


def gen_replacement_filter_from_dict(names_dict: dict):
    def func(name: str):
        return names_dict.get(name.upper().replace("-", "_").replace(" ", "_"), "")

    return func


icons_filter = gen_replacement_filter_from_dict(ICONS)


register.filter("name", gen_replacement_filter_from_dict(NAMES))
register.filter("icon", icons_filter)


@register.filter
def large_icon(name):
    return icons_filter(name).replace("-small", "-large")


ENVIRONMENT_DAMAGE_CAUSES = [
    "FIRE",
    "LAVA",
    "THUNDERBOLT",
    "CACTUS",
    "TNT",
    "FALL",
    "SUFFOCATION",
    "DROWNING",
    "STARVATION",
    "COMMAND",
    "UNKNOWN",
]


@register.filter
def is_creature(damage_cause: str):
    return (
        damage_cause.upper().replace("-", "_").replace(" ", "_")
        not in ENVIRONMENT_DAMAGE_CAUSES
    )


def gen_filtering_dict_filter(filter_callable):
    def func(unfiltered_dict):
        for key, value in unfiltered_dict.items():
            if filter_callable(key):
                yield key, value

    return func


register.filter(
    "only_creatures",
    gen_filtering_dict_filter(lambda cause: cause not in ENVIRONMENT_DAMAGE_CAUSES),
)
register.filter(
    "only_environment",
    gen_filtering_dict_filter(lambda cause: cause in ENVIRONMENT_DAMAGE_CAUSES),
)
