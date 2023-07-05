from dataclasses import dataclass
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
    postgres: PostgresConfig | None = None


def load_config(path: str) -> Config | None:
    file_path = Path(path)
    config = dacite.from_dict(
        data_class=Config,
        data=toml.load(file_path)
    )
    return config
