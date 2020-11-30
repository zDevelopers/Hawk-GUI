import json
import os
from uuid import UUID

from Levenshtein import distance
from django import template
from django.conf import settings
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
    # Statistics
    "minecraft.leave_game": _("Games quit"),
    "minecraft.play_one_minute": _("Time Played"),
    "minecraft.time_since_death": _("Since Last Death"),
    "minecraft.sneak_time": _("Sneak Time"),
    "minecraft.walk_one_cm": _("Distance Walked"),
    "minecraft.crouch_one_cm": _("Distance Crouched"),
    "minecraft.sprint_one_cm": _("Distance Sprinted"),
    "minecraft.swim_one_cm": _("Distance Swum"),
    "minecraft.fall_one_cm": _("Distance Fallen"),
    "minecraft.climb_one_cm": _("Distance Climbed"),
    "minecraft.fly_one_cm": _("Distance Flown"),
    "minecraft.walk_under_water_one_cm": _("Distance Walked under Water"),
    "minecraft.dive_one_cm": _("Distance Walked under Water"),  # legacy
    "minecraft.minecart_one_cm": _("Distance by Minecart"),
    "minecraft.boat_one_cm": _("Distance by Boat"),
    "minecraft.pig_one_cm": _("Distance by Pig"),
    "minecraft.horse_one_cm": _("Distance by Horse"),
    "minecraft.aviate_one_cm": _("Distance by Elytra"),
    "minecraft.jump": _("Jumps"),
    "minecraft.damage_dealt": _("Damage Dealt"),
    "minecraft.damage_dealt_absorbed": _("Damage Dealt (Absorbed)"),
    "minecraft.damage_dealt_resisted": _("Damage Dealt (Resisted)"),
    "minecraft.damage_taken": _("Damage Taken"),
    "minecraft.damage_absorbed": _("Damage Absorbed"),
    "minecraft.damage_resisted": _("Damage Resisted"),
    "minecraft.damage_blocked_by_shield": _("Damage Blocked by Shield"),
    "minecraft.deaths": _("Number of Deaths"),
    "minecraft.mob_kills": _("Mob Kills"),
    "minecraft.player_kills": _("Player Kills"),
    "minecraft.drop": _("Items Dropped"),
    "minecraft.enchant_item": _("Items Enchanted"),
    "minecraft.animals_bred": _("Animals Bred"),
    "minecraft.eat": _("Ate"),  # Legacy
    "minecraft.fish_caught": _("Fish Caught"),
    "minecraft.talked_to_villager": _("Talked to Villagers"),
    "minecraft.traded_with_villager": _("Traded with Villagers"),
    "minecraft.eat_cake_slice": _("Cake Slices Eaten"),
    "minecraft.fill_cauldron": _("Cauldrons Filled"),
    "minecraft.use_cauldron": _("Water Taken from Cauldron"),
    "minecraft.clean_armor": _("Armor Pieces Cleaned"),
    "minecraft.clean_banner": _("Banners Cleaned"),
    "minecraft.interact_with_brewingstand": _("Brewing Stand Used"),
    "minecraft.interact_with_beacon": _("Beacon Used"),
    "minecraft.interact_with_crafting_table": _("Crafting Table Used"),
    "minecraft.interact_with_furnace": _("Furnace Used"),
    "minecraft.interact_with_blast_furnace": _("Blast Furnace Used"),
    "minecraft.interact_with_campfire": _("Campfire Used"),
    "minecraft.interact_with_cartography_table": _("Cartography Table Used"),
    "minecraft.interact_with_lectern": _("Lectern Used"),
    "minecraft.interact_with_loom": _("Loom Used"),
    "minecraft.interact_with_smoker": _("Smoker Used"),
    "minecraft.interact_with_stonecutter": _("Stonecutter Used"),
    "minecraft.inspect_dispenser": _("Dispensers Searched"),
    "minecraft.inspect_dropper": _("Droppers Searched"),
    "minecraft.inspect_hopper": _("Hoppers Searched"),
    "minecraft.open_chest": _("Chests Opened"),
    "minecraft.trigger_trapped_chest": _("Trapped Chests Triggered"),
    "minecraft.open_enderchest": _("Ender Chests Opened"),
    "minecraft.play_noteblock": _("Note Blocks played"),
    "minecraft.tune_noteblock": _("Note Blocks tuned"),
    "minecraft.pot_flower": _("Plants Potted"),
    "minecraft.play_record": _("Music Discs Played"),
    "minecraft.sleep_in_bed": _("Times Slept in a Bed"),
    "minecraft.clean_shulker_box": _("Shulker Boxes Cleaned"),
    "minecraft.open_shulker_box": _("Shulker Boxes Opened"),
    "minecraft.walk_on_water_one_cm": _("Distance Walked on Water"),
    "minecraft.time_since_rest": _("Since Last Rest"),
    "minecraft.open_barrel": _("Barrels Opened"),
    "minecraft.bell_ring": _("Bells Rung"),
    "minecraft.raid_trigger": _("Raids Triggered"),
    "minecraft.raid_win": _("Raids Won"),
}

DESCRIPTIONS = {
    "minecraft.leave_game": _(
        'The number of times "Save and quit to title" has been clicked.'
    ),
    "minecraft.play_one_minute": _("The total amount of time played."),
    "minecraft.time_since_death": _("The time since the player's last death."),
    "minecraft.sneak_time": _("The time the player has held down the sneak button."),
    "minecraft.walk_one_cm": _("The total distance walked."),
    "minecraft.crouch_one_cm": _("The total distance walked while sneaking."),
    "minecraft.sprint_one_cm": _("The total distance sprinted."),
    "minecraft.swim_one_cm": _("The total distance covered with sprint-swimming."),
    "minecraft.fall_one_cm": _(
        "The total distance fallen, excluding jumping. If the player falls more than one block, the entire jump is "
        "counted."
    ),
    "minecraft.climb_one_cm": _("The total distance traveled up ladders or vines."),
    "minecraft.fly_one_cm": _(
        "The total distance traveled upwards and forwards at the same time, while more than one block above the ground."
    ),
    "minecraft.walk_under_water_one_cm": _("The total distance walked underwater."),
    "minecraft.dive_one_cm": _("The total distance walked underwater."),  # legacy
    "minecraft.minecart_one_cm": _("The total distance traveled by minecarts."),
    "minecraft.boat_one_cm": _("The total distance traveled by boats."),
    "minecraft.pig_one_cm": _("The total distance traveled by pigs via saddles."),
    "minecraft.horse_one_cm": _("The total distance traveled by horses."),
    "minecraft.aviate_one_cm": _("The total distance traveled by elytra."),
    "minecraft.jump": _("The total number of jumps performed."),
    "minecraft.damage_dealt": _(
        "The amount of damage the player has dealt (including towards mobs). Only includes melee attacks."
    ),
    "minecraft.damage_dealt_absorbed": _(
        "The amount of damage the player has dealt that were absorbed."
    ),
    "minecraft.damage_dealt_resisted": _(
        "The amount of damage the player has dealt that were resisted."
    ),
    "minecraft.damage_taken": _("The amount of damage the player has taken."),
    "minecraft.damage_absorbed": _(
        "The amount of damage the player has absorbed (e.g. via the Absorption potion effect)."
    ),
    "minecraft.damage_resisted": _(
        "The amount of damage the player has resisted (e.g. using an armor)."
    ),
    "minecraft.damage_blocked_by_shield": _(
        "The amount of damage the player has blocked with a shield."
    ),
    "minecraft.deaths": _("The number of times the player died."),
    "minecraft.mob_kills": _("The number of mobs the player killed."),
    "minecraft.player_kills": _(
        "The number of players the player killed. Indirect killings (such as pushing people off a cliff) do not count."
    ),
    "minecraft.drop": _(
        "The number of items dropped. This does not include items dropped upon death."
    ),
    "minecraft.enchant_item": _("The number of items enchanted."),
    "minecraft.animals_bred": _("The number of times the player bred two mobs."),
    "minecraft.eat": _("The number of times the player ate."),  # Legacy
    "minecraft.fish_caught": _("The number of fish caught."),
    "minecraft.talked_to_villager": _(
        "The number of times interacted with villagers (opened the trading GUI)."
    ),
    "minecraft.traded_with_villager": _("The number of times traded with villagers."),
    "minecraft.eat_cake_slice": _("The number of cake slices eaten."),
    "minecraft.fill_cauldron": _(
        "The number of times the player filled cauldrons with water buckets."
    ),
    "minecraft.use_cauldron": _(
        "The number of times the player took water from cauldrons with glass bottles."
    ),
    "minecraft.clean_armor": _(
        "The number of dyed leather armors washed with a cauldron."
    ),
    "minecraft.clean_banner": _(
        "The number of banner patterns washed with a cauldron."
    ),
    "minecraft.interact_with_brewingstand": _(
        "The number of times interacted with brewing stands."
    ),
    "minecraft.interact_with_beacon": _("The number of times interacted with beacons."),
    "minecraft.interact_with_crafting_table": _(
        "The number of times interacted with crafting tables."
    ),
    "minecraft.interact_with_furnace": _(
        "The number of times interacted with furnaces."
    ),
    "minecraft.interact_with_blast_furnace": _(
        "The number of times interacted with blast furnaces."
    ),
    "minecraft.interact_with_campfire": _(
        "The number of times interacted with campfires."
    ),
    "minecraft.interact_with_cartography_table": _(
        "The number of times interacted with cartography tables."
    ),
    "minecraft.interact_with_lectern": _(
        "The number of times interacted with lecterns."
    ),
    "minecraft.interact_with_loom": _("The number of times interacted with looms."),
    "minecraft.interact_with_smoker": _("The number of times interacted with smokers."),
    "minecraft.interact_with_stonecutter": _(
        "The number of times interacted with stonecutters."
    ),
    "minecraft.inspect_dispenser": _("The number of times interacted with dispensers."),
    "minecraft.inspect_dropper": _("The number of times interacted with droppers."),
    "minecraft.inspect_hopper": _("The number of times interacted with hoppers."),
    "minecraft.open_chest": _("The number of times the player opened chests."),
    "minecraft.trigger_trapped_chest": _(
        "The number of times the player opened trapped chests."
    ),
    "minecraft.open_enderchest": _(
        "The number of times the player opened ender chests."
    ),
    "minecraft.play_noteblock": _("The number of note blocks hit."),
    "minecraft.tune_noteblock": _("The number of times interacted with note blocks."),
    "minecraft.pot_flower": _("The number of plants potted onto flower pots."),
    "minecraft.play_record": _("The number of music discs played on a jukebox."),
    "minecraft.sleep_in_bed": _("The number of times the player has slept in a bed."),
    "minecraft.clean_shulker_box": _(
        "The number of times the player has washed a Shulker Box with a cauldron."
    ),
    "minecraft.open_shulker_box": _(
        "The number of times the player has opened a Shulker Box."
    ),
    "minecraft.walk_on_water_one_cm": _(
        "The distance covered while bobbing up and down over water."
    ),
    "minecraft.time_since_rest": _("The time since the player's last rest."),
    "minecraft.open_barrel": _("The number of times the player has opened a barrel."),
    "minecraft.bell_ring": _("The number of times the player has rung a bell."),
    "minecraft.raid_trigger": _("The number of times the player has triggered a raid."),
    "minecraft.raid_win": _("The number of times the player has won a raid."),
}

ICONS = {
    # Damage causes
    "ZOMBIE": "entity-zombie",
    "PIGMAN": "entity-zombie-pigman",
    "FIRE": "block-fire",
    "LAVA": "block-lava",
    "THUNDERBOLT": "entity-lightning",
    "CACTUS": "block-cactus-side",
    "TNT": "block-tnt-side",
    "FALL": "block-stone",
    "SUFFOCATION": "block-sand",
    "DROWNING": "block-water",
    "STARVATION": "item-rotten-flesh",
    "COMMAND": "block-command-block-back",
    # Weapons
    "SWORD_WOOD": "item-wood-sword",
    "SWORD_STONE": "item-stone-sword",
    "SWORD_IRON": "item-iron-sword",
    "SWORD_GOLD": "item-gold-sword",
    "SWORD_DIAMOND": "item-diamond-sword",
    "AXE_WOOD": "item-wood-axe",
    "AXE_STONE": "item-stone-axe",
    "AXE_IRON": "item-iron-axe",
    "AXE_GOLD": "item-gold-axe",
    "AXE_DIAMOND": "item-diamond-axe",
    "PICKAXE_WOOD": "item-wood-pickaxe",
    "PICKAXE_STONE": "item-stone-pickaxe",
    "PICKAXE_IRON": "item-iron-pickaxe",
    "PICKAXE_GOLD": "item-gold-pickaxe",
    "PICKAXE_DIAMOND": "item-diamond-pickaxe",
    "HOE_WOOD": "item-wood-hoe",
    "HOE_STONE": "item-stone-hoe",
    "HOE_IRON": "item-iron-hoe",
    "HOE_GOLD": "item-gold-hoe",
    "HOE_DIAMOND": "item-diamond-hoe",
    "SHOVEL_WOOD": "item-wood-shovel",
    "SHOVEL_STONE": "item-stone-shovel",
    "SHOVEL_IRON": "item-iron-shovel",
    "SHOVEL_GOLD": "item-gold-shovel",
    "SHOVEL_DIAMOND": "item-diamond-shovel",
    "BOW": "item-bow-pulling",
    "MAGIC": "item-potion",
    "THORNS": "item-diamond-chestplate",
    # Healing causes
    "NATURAL": "item-baked-potato",
    "GOLDEN_APPLE": "item-golden-apple",
    "NOTCH_APPLE": "item-golden-apple",
    "HEALING_POTION": "item-potion",
    # Others
    "UNKNOWN": "entity-unknown",
    # Statistics
    "minecraft.leave_game": "item-barrier",
    "minecraft.play_one_minute": "item-clock",
    "minecraft.time_since_death": "item-totem-of-undying",
    "minecraft.sneak_time": "item-empty-armor-slot-boots",
    "minecraft.walk_one_cm": "item-iron-boots",
    "minecraft.crouch_one_cm": "item-empty-armor-slot-boots",
    "minecraft.sprint_one_cm": "item-diamond-boots",
    "minecraft.swim_one_cm": "item-cod",
    "minecraft.fall_one_cm": "item-feather",
    "minecraft.climb_one_cm": "block-ladder",
    "minecraft.fly_one_cm": "entity-bat",
    "minecraft.walk_under_water_one_cm": "block-tall-seagrass-bottom",
    "minecraft.dive_one_cm": "block-tall-seagrass-bottom",  # legacy
    "minecraft.minecart_one_cm": "item-minecart",
    "minecraft.boat_one_cm": "item-birch-boat",
    "minecraft.pig_one_cm": "item-carrot-on-a-stick",
    "minecraft.horse_one_cm": "entity-horse",
    "minecraft.aviate_one_cm": "item-elytra",
    "minecraft.jump": "item-rabbit-foot",
    "minecraft.damage_dealt": "item-diamond-sword",
    "minecraft.damage_dealt_absorbed": "item-diamond-chestplate",
    "minecraft.damage_dealt_resisted": "item-empty-armor-slot-shield",
    "minecraft.damage_taken": "item-iron-sword",
    "minecraft.damage_absorbed": "item-iron-chestplate",
    "minecraft.damage_resisted": "item-empty-armor-slot-shield",
    "minecraft.damage_blocked_by_shield": "item-empty-armor-slot-shield",  # TODO shield icon
    "minecraft.deaths": "block-wither-rose",
    "minecraft.mob_kills": "item-iron-axe",
    "minecraft.player_kills": "item-diamond-axe",
    "minecraft.drops": "item-brick",
    "minecraft.enchant_item": "item-enchanted-book",
    "minecraft.animals_bread": "entity-chicken",
    "minecraft.eat": "item-baked-potato",
    "minecraft.fish_caught": "entity-fishing-rod",
    "minecraft.talked_to_villager": "entity-villager",
    "minecraft.traded_with_villager": "entity-wandering-trader",
    "minecraft.eat_cake_slice": "item-cake",
    "minecraft.fill_cauldron": "block-cauldron-side",
    "minecraft.use_cauldron": "block-cauldron-top",
    "minecraft.clean_armor": "item-leather-chestplate",
    "minecraft.clean_banner": "item-mojang-banner-pattern",
    "minecraft.interact_with_brewingstand": "item-brewing-stand",
    "minecraft.interact_with_beacon": "block-beacon",
    "minecraft.interact_with_crafting_table": "block-crafting-table-front",
    "minecraft.interact_with_furnace": "block-furnace-front-on",
    "minecraft.interact_with_blast_furnace": "block-blast-furnace-front-on",
    "minecraft.interact_with_campfire": "item-campfire",
    "minecraft.interact_with_cartography_table": "block-cartography-table-top",
    "minecraft.interact_with_lectern": "item-writable-book",
    "minecraft.interact_with_loom": "block-loom-front",
    "minecraft.interact_with_smoker": "block-smoker-front-on",
    "minecraft.interact_with_stonecutter": "block-stonecutter-saw",
    "minecraft.inspect_dispenser": "block-dispenser-front",
    "minecraft.inspect_dropper": "block-dropper-front",
    "minecraft.inspect_hopper": "item-hopper",
    "minecraft.open_chest": "block-barrel-side",
    "minecraft.trigger_trapped_chest": "block-tripwire-hook",
    "minecraft.open_enderchest": "block-obsidian",
    "minecraft.play_noteblock": "block-note-block",
    "minecraft.tune_noteblock": "block-note-block",
    "minecraft.pot_flower": "block-flower-pot",
    "minecraft.play_record": "item-music-disc-13",
    "minecraft.sleep_in_bed": "item-lantern",
    "minecraft.clean_shulker_box": "block-shulker-box",
    "minecraft.open_shulker_box": "block-lime-shulker-box",
    "minecraft.walk_on_water_one_cm": "item-seagrass",
    "minecraft.time_since_rest": "entity-phantom",
    "minecraft.open_barrel": "block-barrel-top-open",
    "minecraft.bell_ring": "item-bell",
    "minecraft.raid_trigger": "entity-ravager",
    "minecraft.raid_win": "entity-rabbit",
}

for thing in NAMES.keys():
    if thing not in ICONS:
        ICONS[thing] = f"entity-{thing.lower().replace('_', '-')}"

try:
    with open(os.path.join(settings.BASE_DIR, "static", "sprites", "icons.json")) as f:
        AVAILABLE_ICONS = json.load(f)["icons"]
except (KeyError, FileNotFoundError):
    AVAILABLE_ICONS = []

# This is used as a cache to avoid computing icons from name multiple times, as it's
# a heavy operation. We pre-populate the cache with initial associations that our
# engine cannot catch as they are not name-based (e.g. textures named with a synonym).
AVAILABLE_ICONS_ASSOCIATION_CACHE = {
    "tallgrass": "block-tall-grass-top",
    "waterlily": "block-lily-pad",
    "reeds": "block-sugar-cane",
    "furnace": "block-furnace-front",
    "nether-brick": "block-nether-bricks",
    "nether-brick-fence": "block-nether-bricks",
}


def normalize_dict_keys(the_dict: dict):
    return {key.lower().strip(): value for key, value in the_dict.items()}


NAMES = normalize_dict_keys(NAMES)
ICONS = normalize_dict_keys(ICONS)


def get_from_dict_messy(the_dict, key):
    return the_dict.get(key.lower().replace("-", "_").replace(" ", "_"), "")


@register.filter
def name(key):
    name = get_from_dict_messy(NAMES, key)
    if not name:
        name = (
            key.lower()
            .replace("minecraft.", "")
            .replace("minecraft:", "")
            .replace("_", " ")
            .replace("-", " ")
            .replace(".", " ")
            .capitalize()
        )
    return name


@register.filter
def description(key):
    return get_from_dict_messy(DESCRIPTIONS, key)


@register.filter
def icon(key, size=None):
    icon = get_from_dict_messy(ICONS, key)

    # If not found we try to guess from all available icons
    if not icon:
        if key.startswith("minecraft:") or key.startswith("minecraft."):
            key = key[10:]
        key = key.replace("_", "-").lower()
        if key in AVAILABLE_ICONS_ASSOCIATION_CACHE:
            icon = AVAILABLE_ICONS_ASSOCIATION_CACHE[key]
        else:
            potential_icons = []
            key_parts = key.split("-")

            # We first extract all icons containing the key, and then use the one
            # with the closer name (closer Levenshtein distance).
            for i in AVAILABLE_ICONS:
                if i.endswith("-small") or i.endswith("-large"):
                    continue
                if key in i or any(
                    [
                        part in i
                        for part in key_parts
                        if part not in ["block", "item", "entity"]
                    ]
                ):
                    potential_icons.append(i)

            if potential_icons:
                closest = (-1, None)
                for i in potential_icons:
                    levenshtein_distance = distance(i, key)
                    if closest[0] == -1 or levenshtein_distance < closest[0]:
                        closest = (levenshtein_distance, i)

                icon = closest[1]
                AVAILABLE_ICONS_ASSOCIATION_CACHE[key] = closest[1]

    if not icon:
        icon = "entity-unknown"

    if size is not None:
        icon += f"-{size}"

    return icon


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
