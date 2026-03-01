#!/usr/bin/env python3
"""Ensure docs/indicator_registry.json covers the Rust public indicator surface."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
REGISTRY_PATH = ROOT / "docs" / "indicator_registry.json"

INDICATOR_MODULES = {
    "basic_indicators": ROOT / "src" / "basic_indicators.rs",
    "candle_indicators": ROOT / "src" / "candle_indicators.rs",
    "chart_trends": ROOT / "src" / "chart_trends.rs",
    "correlation_indicators": ROOT / "src" / "correlation_indicators.rs",
    "momentum_indicators": ROOT / "src" / "momentum_indicators.rs",
    "moving_average": ROOT / "src" / "moving_average.rs",
    "other_indicators": ROOT / "src" / "other_indicators.rs",
    "strength_indicators": ROOT / "src" / "strength_indicators.rs",
    "trend_indicators": ROOT / "src" / "trend_indicators.rs",
    "volatility_indicators": ROOT / "src" / "volatility_indicators.rs",
}

PUB_MOD_RE = re.compile(r"^\s*pub\s+mod\s+(single|bulk)\s*\{")
PUB_FN_RE = re.compile(r"^\s*pub\s+fn\s+([a-zA-Z0-9_]+)\s*\(")


def extract_public_functions(module: str, path: Path) -> set[str]:
    """Return expected function_path entries for one module file."""
    lines = path.read_text().splitlines()
    brace_depth = 0
    current_mode: str | None = None
    mode_depth = -1

    function_paths: set[str] = set()

    for line in lines:
        mod_match = PUB_MOD_RE.match(line)
        if mod_match:
            current_mode = mod_match.group(1)
            mode_depth = brace_depth + 1

        fn_match = PUB_FN_RE.match(line)
        if fn_match:
            fn_name = fn_match.group(1)
            if current_mode in ("single", "bulk"):
                function_paths.add(
                    f"centaur_technical_indicators::{module}::{current_mode}::{fn_name}"
                )
            elif module == "chart_trends":
                function_paths.add(f"centaur_technical_indicators::{module}::{fn_name}")

        brace_depth += line.count("{")
        brace_depth -= line.count("}")

        if current_mode and brace_depth < mode_depth:
            current_mode = None
            mode_depth = -1

    return function_paths


def main() -> int:
    registry = json.loads(REGISTRY_PATH.read_text())
    registry_paths = {entry["function_path"] for entry in registry["indicators"]}

    expected_paths: set[str] = set()
    for module, module_path in INDICATOR_MODULES.items():
        expected_paths.update(extract_public_functions(module, module_path))

    missing_in_registry = sorted(expected_paths - registry_paths)
    extra_in_registry = sorted(registry_paths - expected_paths)

    if missing_in_registry or extra_in_registry:
        print("indicator_registry coverage check failed.")
        if missing_in_registry:
            print("\nMissing from docs/indicator_registry.json:")
            for path in missing_in_registry:
                print(f"  - {path}")
        if extra_in_registry:
            print("\nPresent in docs/indicator_registry.json but not in src indicators:")
            for path in extra_in_registry:
                print(f"  - {path}")
        return 1

    print("indicator_registry coverage check passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
