import json
import os
import re
from pathlib import Path


BASE_DIR = Path(__file__).resolve().parent
TEMPLATE_PATH = BASE_DIR / "settings.templ.json"
ENV_PATH = BASE_DIR / ".env"
OUTPUT_PATH = BASE_DIR / "config/settings.json"
ENV_VAR_PATTERN = re.compile(r"\$\{([^}]+)\}")


def load_env_file(env_path: Path) -> None:
    if not env_path.is_file():
        return

    for line in env_path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue

        key, value = line.split("=", 1)
        os.environ.setdefault(key.strip(), value.strip())


def resolve_placeholders(value):
    if isinstance(value, dict):
        return {key: resolve_placeholders(item) for key, item in value.items()}
    if isinstance(value, list):
        return [resolve_placeholders(item) for item in value]
    if not isinstance(value, str):
        return value

    def replace_env_var(match: re.Match[str]) -> str:
        env_var = match.group(1)
        env_value = os.environ.get(env_var)
        if env_value is None:
            raise ValueError(f"Missing environment variable: {env_var}")
        return env_value

    return ENV_VAR_PATTERN.sub(replace_env_var, value)


def main() -> None:
    load_env_file(ENV_PATH)

    with TEMPLATE_PATH.open(encoding="utf-8") as template_file:
        config = json.load(template_file)

    rendered_config = resolve_placeholders(config)

    with OUTPUT_PATH.open("w", encoding="utf-8") as output_file:
        json.dump(rendered_config, output_file, indent=2)
        output_file.write("\n")

    print(f"Wrote {OUTPUT_PATH}")


if __name__ == "__main__":
    main()
