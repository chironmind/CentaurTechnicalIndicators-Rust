#!/usr/bin/env python3
"""Check docs for common naming drift and typo patterns."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
README_PATH = ROOT / "README.md"
REGISTRY_PATH = ROOT / "docs" / "indicator_registry.json"

DOC_FILES = [
    ROOT / "README.md",
    ROOT / "CONTRIBUTING.md",
    ROOT / "AGENTS.md",
    ROOT / "AI_FRIENDLY_ROADMAP.md",
    ROOT / "docs" / "REPO_MAP.md",
    ROOT / "src" / "momentum_indicators.rs",
    ROOT / "src" / "chart_trends.rs",
]

FORBIDDEN_TOKENS = {
    "defaut_rsi": "default_rsi",
    "Programatically": "Programmatically",
    "donchian_channel": "donchian_channels",
    "true_strength_indx": "true_strength_index",
    "absoluite_deviation": "absolute_deviation",
}

BENCHMARK_NAME_RE = re.compile(r"^\|\s*`([a-z0-9_]+)`\s*\|")


def check_forbidden_tokens() -> list[str]:
    errors: list[str] = []
    for path in DOC_FILES:
        text = path.read_text()
        for token, replacement in FORBIDDEN_TOKENS.items():
            token_re = re.compile(rf"\\b{re.escape(token)}\\b")
            if token_re.search(text):
                errors.append(
                    f"{path.relative_to(ROOT)} contains '{token}' (use '{replacement}')"
                )
    return errors


def check_readme_benchmark_function_names() -> list[str]:
    registry = json.loads(REGISTRY_PATH.read_text())
    known_function_names = {
        item["function_path"].rsplit("::", maxsplit=1)[-1] for item in registry["indicators"]
    }

    errors: list[str] = []
    for line_num, line in enumerate(README_PATH.read_text().splitlines(), start=1):
        match = BENCHMARK_NAME_RE.match(line)
        if not match:
            continue
        fn_name = match.group(1)
        if fn_name not in known_function_names:
            errors.append(f"README.md:{line_num} references unknown function '{fn_name}'")
    return errors


def main() -> int:
    errors = []
    errors.extend(check_forbidden_tokens())
    errors.extend(check_readme_benchmark_function_names())
    if errors:
        print("Docs consistency check failed:")
        for error in errors:
            print(f"  - {error}")
        return 1
    print("Docs consistency check passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
