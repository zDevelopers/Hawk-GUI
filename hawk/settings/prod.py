import toml
import pymysql  # noqa - only installed on the production server

from pathlib import Path

from .base import *  # noqa

config_path = os.environ.get("HAWK_CONFIG", str(BASE_DIR / "config.toml"))

try:
    config = toml.load(config_path)
    print(f"Using the config file at {config_path!r}")
except OSError:
    config = {}

DEBUG = False

ALLOWED_HOSTS = config.get("allowed_hosts", ['hawk.carrade.eu'])
SECRET_KEY = config["secret_key"]

CONTENTS_DIR = Path(config.get("contents_dir", BASE_DIR / ".." / "hawk-data"))
LOGS_DIR = Path(config.get("logs_dir", CONTENTS_DIR / "logs"))

DATABASES = {
    "default": {
        "ENGINE": "django.db.backends.mysql",
        "NAME": config["databases"]["default"].get("name", "hawk"),
        "USER": config["databases"]["default"].get("user", "hawk"),
        "PASSWORD": config["databases"]["default"]["password"],
        "HOST": config["databases"]["default"].get("host", ""),
        "PORT": config["databases"]["default"].get("port", ""),
        "CONN_MAX_AGE": 600,
        "init_command": "SET sql_modes = 'STRICT_TRANS_TABLES'",
        "OPTIONS": {
            "charset": "utf8mb4",
        },
    },
}

# Enable PyMySQL compatibility layer, as Django expect MySQLdb (from
# mysqlclient), but it's a pain to install on CentOS.
pymysql.version_info = (2, 0, 3, "final", 0)
pymysql.install_as_MySQLdb()

LOGGING = {
    'version': 1,
    'disable_existing_loggers': False,
    'handlers': {
        'file': {
            'level': 'DEBUG',
            'class': 'logging.FileHandler',
            'filename': LOGS_DIR / "django.log",
        },
    },
    'root': {
        'handlers': ['file'],
        'level': 'DEBUG',
    },
}

STATIC_ROOT = CONTENTS_DIR / "static"
MEDIA_ROOT = CONTENTS_DIR / "user-generated-content"
MAINTENANCE_MODE_STATE_FILE_PATH = CONTENTS_DIR / "maintenance"
