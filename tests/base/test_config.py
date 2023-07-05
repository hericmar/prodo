import unittest

import dacite
import toml.decoder

from base.config import Config

INPUT = """
debug = true

allowed_hosts = ["localhost", "example.com"]
csrf_trusted_origins = ["http://localhost", "https://example.com"]
"""


class TestConfig(unittest.TestCase):

    def test_config_deserialization(self):
        config = dacite.from_dict(
            data_class=Config,
            data=toml.loads(INPUT)
        )
        print(config)


if __name__ == "__main__":
    unittest.main()
