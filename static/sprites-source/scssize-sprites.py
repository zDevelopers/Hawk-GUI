"""
Converts the CSS file from http://css.spritegen.com to a SCSS map.

Batch-resize keeping pixels:
$ convert "*.png" -scale 200% -set filename:base "%[basename]" "images/%[filename:base]-large.png"
"""
import sys

sprites = {}

if len(sys.argv) < 2:
    print("Converts the CSS file from http://css.spritegen.com to a SCSS map.", file=sys.stderr)
    print(f"Usage: {sys.argv[0]} <CSS file>", file=sys.stderr)
    sys.exit(1)

with open(sys.argv[1], 'r') as f:
    for line in f:
        if not line.strip() or ',' in line or '{' not in line or not line.startswith('.'):
            continue

        parts = line.split("{")
        name = parts[0].strip().replace('.', '')

        position = "0 0"
        width = "0"
        height = "0"

        for prop in parts[1].replace('}', '').strip().split(';'):
            prop = prop.split(':')
            prop_name = prop[0].strip()
            if prop_name == 'background-position':
                position = prop[1].strip()
            elif prop_name == 'width':
                width = prop[1].strip()
            elif prop_name == 'height':
                height = prop[1].strip()

        sprites[name] = {
            'position': position,
            'width': width,
            'height': height
        }

print("$sprite: (")

for sprite_name, sprite_props in sprites.items():
    print(f'        "{sprite_name}": ("position": {sprite_props["position"]}, "width": {sprite_props["width"]}, "height": {sprite_props["height"]}),')

print(");")
