import json
import os
import subprocess
import tempfile

from pathlib import Path
from shutil import copyfile
from zipfile import ZipFile, BadZipFile

from PIL import Image
from django.conf import settings
from django.core.management.base import BaseCommand, CommandError


# Aliases for compatibility with old reports
SPRITE_ALIASES = {
    "item-enchanted-book": ["item-book-enchanted"],
    "item-golden-apple": ["item-apple-golden"],
    "block-nether-portal": ["block-portal"],
}

# Some icons are saved multiple times in resources packs, for various
# states (e.g. each compass direction); this specifies which one to keep
# and under which name.
RESOURCES_PACK_REPLACEMENTS = [
    ("destroy_stage", "destroy_stage_9.png", "destroy-stage.png"),
    ("frosted_ice", "frosted_ice_0.png", "frosted-ice.png"),
    ("fire", "fire_1.png", "fire.png"),
    ("lava", "lava_still.png", "lava.png"),
    ("water", "water_still.png", "water.png"),
    ("compass", "compass_19.png", "compass.png"),
    ("clock", "clock_04.png", "clock.png"),
    ("bow_pulling", "bow_pulling_1.png", "bow-pulling.png"),
    ("crossbow_pulling", "crossbow_pulling_2.png", "crossbow-pulling.png"),
]

# In the resources pack, some textures are animated, and to do so
# multiples versions are stacked into the file; textures in this list
# will be crop to the first 16×16 square at the top.
RESOURCES_PACK_TRIM_ANIMATIONS = [
    "kelp.png",
    "stonecutter-saw.png",
    "smoker-front-on.png",
    "lantern.png",
    "repeating-command-block-back.png",
    "water.png",
    "seagrass.png",
    "repeating-command-block-front.png",
    "sea-lantern.png",
    "tall-seagrass-top.png",
    "kelp-plant.png",
    "magma.png",
    "prismarine.png",
    "lava.png",
    "nether-portal.png",
    "tall-seagrass-bottom.png",
    "repeating-command-block-side.png",
    "repeating-command-block-conditional.png",
    "command-block-back.png",
    "chain-command-block-back.png",
    "chain-command-block-side.png",
    "fire.png",
    "blast-furnace-front-on.png",
    "command-block-conditional.png",
    "chain-command-block-conditional.png",
    "command-block-side.png",
    "campfire-fire.png",
    "chain-command-block-front.png",
    "command-block-front.png",
    "campfire-log-lit.png",
]

# Not all icons are available in large format, to get a smaller sprite.
# All entities are, plus these ones. We use them to display damages source
# summaries.
NON_ENTITIES_LARGE_ICONS = [
    "block-fire.png",
    "block-lava.png",
    "block-cactus-side.png",
    "block-tnt-side.png",
    "block-stone.png",
    "block-sand.png",
    "block-water.png",
    "item-rotten-flesh.png",
    "block-command-block-back.png",
    "block-campfire-fire.png",
    "block-campfire-log-lit.png",
    "item-broken-elytra.png",
]


class Command(BaseCommand):
    help = (
        "Generates the sprite (image + SCSS) from a resource pack and a folder with entities icons. "
        "Requires development dependencies, `convert`, and `optipng` without --fast. "
    )

    def add_arguments(self, parser):
        parser.add_argument(
            "--resources-pack",
            type=str,
            default=str(
                os.path.join(settings.BASE_DIR, "static", "sprites", "resources.zip")
            ),
            help="A Minecraft resources pack to use as an icons/blocks source. (Default: %(default)s)",
        )
        parser.add_argument(
            "--images",
            type=str,
            default=str(os.path.join(settings.BASE_DIR, "static", "sprites", "images")),
            help="A folder containing remaining images required for sprite generation, "
            "namely entities and some GUI icons. See default for format. (Default: %(default)s)",
        )
        parser.add_argument(
            "--output-sprite",
            type=str,
            default=str(
                os.path.join(
                    settings.BASE_DIR, "static", "dist", "sprite", "hawk-sprite.png"
                )
            ),
            help="The file where the sprite image will be written. (Default: %(default)s)",
        )
        parser.add_argument(
            "--output-scss",
            type=str,
            default=str(
                os.path.join(
                    settings.BASE_DIR,
                    "static",
                    "scss",
                    "reports",
                    "minecraft",
                    "_icons_map.scss",
                )
            ),
            help="The file where the sprite image will be written. (Default: %(default)s)",
        )
        parser.add_argument(
            "--fast",
            action="store_true",
            default=False,
            help="Disables file size optimizations",
        )

    def handle(self, *args, **options):
        resources_pack = Path(options["resources_pack"])
        images = Path(options["images"])

        output_sprite = Path(options["output_sprite"])
        output_scss = Path(options["output_scss"])

        if not resources_pack.exists() or not resources_pack.is_file():
            raise CommandError("The resources pack doesn't exist or is not a file.")
        if not images.exists() or not images.is_dir():
            raise CommandError(
                "The images directory doesn't exist or is not a directory."
            )

        try:
            with tempfile.TemporaryDirectory() as working_dir:
                # We first create some directories to work into.
                # We will unzip the resource pack into `pack` and store all
                # future sprites into `images`.
                working_dir = Path(working_dir)
                working_dir_pack = working_dir / "pack"
                working_dir_sprite_images = working_dir / "images"
                working_dir_sprite_images_final = working_dir_sprite_images / "final"
                working_dir_glue = working_dir / "glue"

                working_dir_pack.mkdir(parents=True)
                working_dir_sprite_images.mkdir(parents=True)
                working_dir_sprite_images_final.mkdir(parents=True)
                working_dir_glue.mkdir(parents=True)

                self.stdout.write("Extracting files…")

                with ZipFile(resources_pack) as pack:
                    pack.extractall(path=working_dir_pack)

                # From the resources pack, we extract blocks and items. But some images needs
                # processing: some are in multiple versions (e.g. all compasses—we only save
                # one of them), and some other are in fact staked frames of an animation
                # (e.g. fire—we crop the file to only save the first frame).
                pack_blocks = (
                    working_dir_pack / "assets" / "minecraft" / "textures" / "block"
                )
                pack_items = (
                    working_dir_pack / "assets" / "minecraft" / "textures" / "item"
                )

                self.stdout.write("Processing icons from resources pack…")

                def get_sprite_name(name: str):
                    sprite_name = name.replace("_", "-")
                    for (
                        start,
                        one_to_keep,
                        replacement_name,
                    ) in RESOURCES_PACK_REPLACEMENTS:
                        if name.startswith(start):
                            if name == one_to_keep:
                                sprite_name = replacement_name
                            else:
                                return None
                            break
                    return sprite_name

                def filter_process_and_move_textures(folder: Path, prefix: str):
                    for texture in folder.glob("*.png"):
                        sprite_name = get_sprite_name(texture.name)
                        if sprite_name:
                            destination = (
                                working_dir_sprite_images / f"{prefix}-{sprite_name}"
                            )
                            texture.rename(destination)
                            if sprite_name in RESOURCES_PACK_TRIM_ANIMATIONS:
                                self.trim_animation(destination)

                filter_process_and_move_textures(pack_blocks, "block")
                filter_process_and_move_textures(pack_items, "item")

                # Now we grab entities from the other images directory. We do not grab
                # `gui` icons because these are not resized, so we will copy them later.

                self.stdout.write("Processing entity icons…")

                large_icons = []

                for entity in (images / "entities").glob("*.png"):
                    sprite_name = f"entity-{entity.name.replace('_', '-')}"
                    copyfile(entity, working_dir_sprite_images / sprite_name)
                    large_icons.append(sprite_name)

                # Now we scale everything by 200%, because all icons are 16×16 and we
                # want normal size to be 32×32.

                self.stdout.write("Scaling icons…")

                subprocess.call(
                    'convert "*.png" -scale 200% -set filename:base "%[basename]" "%[filename:base].png"',
                    shell=True,
                    cwd=working_dir_sprite_images,
                )

                # Then we scale again by 200% for large images (64px), and by 62.5% for small ones (20px).
                # Before, we copy the images to a final sub-folder, to avoid resizing twice while creating
                # small images.

                for image in working_dir_sprite_images.glob("*.png"):
                    copyfile(image, working_dir_sprite_images_final / image.name)

                # We only scale to 64×64 pixels the icons we need at this size
                large_icons.extend(NON_ENTITIES_LARGE_ICONS)
                large_icons_shell_list = " ".join(
                    [f'"{large_icon}"' for large_icon in large_icons]
                )

                subprocess.call(
                    f'convert {large_icons_shell_list} -scale 200% -set filename:base "%[basename]" "final/%[filename:base]-large.png"',
                    shell=True,
                    cwd=working_dir_sprite_images,
                )

                subprocess.call(
                    'convert "*.png" -scale 62% -set filename:base "%[basename]" "final/%[filename:base]-small.png"',
                    shell=True,
                    cwd=working_dir_sprite_images,
                )

                # Now we include the GUI icons we talked about earlier.

                self.stdout.write("Processing GUI icons…")

                for entity in (images / "gui").glob("*.png"):
                    copyfile(
                        entity,
                        working_dir_sprite_images_final
                        / f"gui-{entity.name.replace('_', '-')}",
                    )

                # Time to build the sprite.

                self.stdout.write("Building sprite…")

                subprocess.call(
                    [
                        "glue",
                        "--source",
                        str(working_dir_sprite_images_final),
                        "--output",
                        str(working_dir_glue),
                        "--json",
                        str(working_dir_glue),
                        "--quiet",
                    ]
                )

                # Okay so now we have two files in the `working_dir_glue`: one
                # `final.png`, our sprite, and one `final.json`, the data about
                # the sprite we'll be using to build the SCSS.

                output_scss.parent.mkdir(parents=True, exist_ok=True)
                output_sprite.parent.mkdir(parents=True, exist_ok=True)

                copyfile(working_dir_glue / "final.png", output_sprite)

                # Oh and some optimization along the way (this is slow, yes)
                if not options["fast"]:
                    self.stdout.write(
                        "Optimizing generated sprite… (this can take a long time)"
                    )
                    subprocess.call(["optipng", "-o7", str(output_sprite)])

                self.stdout.write("Generating SCSS…")

                sprite_data = json.load((working_dir_glue / "final.json").open())

                scss = "// File automatically generated by `python manage.py generate_sprite`.\n\n"
                scss += "$hawk_sprite: (\n"

                icons_count = 0
                aliases_count = 0

                for sprite in sprite_data["frames"]:
                    sprite_name = sprite["filename"].replace(".png", "")
                    pos = sprite["frame"]
                    x = f"{pos['x']}px" if pos["x"] != 0 else "0"
                    y = f"{pos['y']}px" if pos["y"] != 0 else "0"
                    w = f"{pos['w']}px"
                    h = f"{pos['h']}px"
                    scss += f'        "{sprite_name}": ("position": {x} {y}, "width": {w}, "height": {h}),\n'

                    icons_count += 1

                    base_sprite_name = sprite_name
                    suffix = ""
                    if sprite_name.endswith("-small"):
                        base_sprite_name = sprite_name[:-6]
                        suffix = "-small"
                    elif sprite_name.endswith("-large"):
                        base_sprite_name = sprite_name[:-6]
                        suffix = "-large"

                    if base_sprite_name in SPRITE_ALIASES:
                        for alias in SPRITE_ALIASES[base_sprite_name]:
                            scss += (
                                f'        "{alias}{suffix}": ("position": {x} {y}, "width": {w}, "height": {h}),  '
                                f"// Alias of {sprite_name}\n"
                            )
                            aliases_count += 1

                scss += ");\n"

                with output_scss.open("w") as scss_file:
                    scss_file.write(scss)

                self.stdout.write(
                    self.style.SUCCESS(
                        f"Done: {icons_count} icons and {aliases_count} aliases (total {icons_count + aliases_count})."
                    )
                )
                self.stdout.write(
                    self.style.SUCCESS(
                        f"Sprite file size: {output_sprite.stat().st_size // 1024} Kio."
                    )
                )

        except BadZipFile as e:
            raise CommandError(f"The resources pack is not a valid ZIP file. {e}")

        except Exception as e:
            raise CommandError("An error occurred", e)

    @staticmethod
    def trim_animation(filename):
        """
        Removes the animation from a Minecraft texture, i.e. only keeps the first
        16x16 pixels of the image.

        :param filename: The filename (works in-place)
        """
        Image.open(filename).crop((0, 0, 16, 16)).save(filename)