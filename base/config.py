from dataclasses import dataclass, field
from pathlib import Path

import dacite
import toml as toml


@dataclass
class PostgresConfig:
    host: str
    port: int
    user: str
    password: str
    database: str


@dataclass
class Config:
    allowed_hosts: list[str] = field(default_factory=lambda: ["localhost"])
    csrf_trusted_origins: list[str] = field(default_factory=lambda: ["http://localhost"])
    postgres: PostgresConfig | None = None


def load_config(path: str) -> Config | None:
    file_path = Path(path)
    config = dacite.from_dict(
        data_class=Config,
        data=toml.load(file_path)
    )
    return config
