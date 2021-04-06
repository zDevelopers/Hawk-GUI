import toml

from .base import *  # noqa

config_path = os.environ.get("HAWK_CONFIG", str(BASE_DIR / "config.toml"))

try:
    config = toml.load(config_path)
    print(f"Using the config file at {config_path!r}")
except OSError:
    config = {}

DEBUG = False
ALLOWED_HOSTS = [
    'hawk.carrade.eu'
]

SECRET_KEY = config["secret_key"]

DATABASES = {
    "default": {
        "ENGINE": "django.db.backends.mysql",
        "NAME": config["databases"]["default"].get("name", "hawk"),
        "USER": config["databases"]["default"].get("user", "hawk"),
        "PASSWORD": config["databases"]["default"]["password"],
        "HOST": config["databases"]["default"].get("host", ""),
        "PORT": config["databases"]["default"].get("port", ""),
        "CONN_MAX_AGE": 600,
        "OPTIONS": {
            "charset": "utf8mb4",
        },
    },
}
