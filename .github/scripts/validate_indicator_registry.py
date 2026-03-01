#!/usr/bin/env python3
"""Lightweight validation for docs/indicator_registry.json against docs/indicator_registry.schema.json."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[2]
SCHEMA_PATH = ROOT / "docs" / "indicator_registry.schema.json"
REGISTRY_PATH = ROOT / "docs" / "indicator_registry.json"


class ValidationError(Exception):
    pass


def expect_type(value: Any, expected: str, path: str) -> None:
    mapping = {
        "object": dict,
        "array": list,
        "string": str,
        "boolean": bool,
        "integer": int,
    }
    py_type = mapping[expected]
    if expected == "integer" and isinstance(value, bool):
        raise ValidationError(f"{path} must be an integer")
    if not isinstance(value, py_type):
        raise ValidationError(f"{path} must be {expected}, got {type(value).__name__}")


def validate(schema: dict[str, Any], value: Any, path: str) -> None:
    schema_type = schema.get("type")
    if schema_type:
        expect_type(value, schema_type, path)

    if schema_type == "object":
        required = schema.get("required", [])
        for key in required:
            if key not in value:
                raise ValidationError(f"{path} missing required field '{key}'")

        additional_allowed = schema.get("additionalProperties", True)
        properties = schema.get("properties", {})
        if additional_allowed is False:
            extras = sorted(set(value) - set(properties))
            if extras:
                raise ValidationError(f"{path} contains unknown fields: {', '.join(extras)}")

        for key, subschema in properties.items():
            if key in value:
                validate(subschema, value[key], f"{path}.{key}")

    if schema_type == "array":
        item_schema = schema.get("items")
        if item_schema:
            for idx, item in enumerate(value):
                validate(item_schema, item, f"{path}[{idx}]")

    if schema_type == "string":
        min_length = schema.get("minLength")
        if min_length is not None and len(value) < min_length:
            raise ValidationError(f"{path} must have at least {min_length} characters")
        pattern = schema.get("pattern")
        if pattern and re.search(pattern, value) is None:
            raise ValidationError(f"{path} does not match required pattern: {pattern}")
        enum = schema.get("enum")
        if enum is not None and value not in enum:
            raise ValidationError(f"{path} must be one of {enum}")

    if schema_type == "integer":
        minimum = schema.get("minimum")
        if minimum is not None and value < minimum:
            raise ValidationError(f"{path} must be >= {minimum}")

    if "oneOf" in schema:
        matches = 0
        for option in schema["oneOf"]:
            try:
                validate(option, value, path)
            except ValidationError:
                continue
            matches += 1
        if matches != 1:
            raise ValidationError(f"{path} must match exactly one oneOf option")


def validate_registry_invariants(registry: dict[str, Any]) -> None:
    indicators = registry["indicators"]
    seen_ids: set[str] = set()
    seen_paths: set[str] = set()
    seen_deprecated_replacements: list[tuple[str, str]] = []

    for idx, indicator in enumerate(indicators):
        path = f"registry.indicators[{idx}]"
        indicator_id = indicator["id"]
        function_path = indicator["function_path"]
        mode = indicator["mode"]
        module = indicator["module"]
        returns = indicator["returns"]
        return_type = returns["return_type"]
        fallible = returns["fallible"]
        source = indicator["source"]
        source_file = ROOT / source["file"]
        source_line = source["line"]
        docs_url = indicator["docs_url"]
        fn_name = function_path.split("::")[-1]
        deprecation = indicator.get("deprecation")
        is_deprecated = indicator["is_deprecated"]

        if indicator_id in seen_ids:
            raise ValidationError(f"{path}.id must be unique; duplicate '{indicator_id}'")
        seen_ids.add(indicator_id)

        if function_path in seen_paths:
            raise ValidationError(
                f"{path}.function_path must be unique; duplicate '{function_path}'"
            )
        seen_paths.add(function_path)

        if not source_file.exists():
            raise ValidationError(f"{path}.source.file does not exist: {source['file']}")
        source_lines = source_file.read_text().splitlines()
        if source_line > len(source_lines):
            raise ValidationError(
                f"{path}.source.line {source_line} exceeds file length {len(source_lines)}"
            )
        source_text = source_lines[source_line - 1]
        if f"pub fn {fn_name}(" not in source_text:
            raise ValidationError(
                f"{path}.source does not point to 'pub fn {fn_name}(' (found: {source_text.strip()})"
            )

        if f"fn.{fn_name}.html" not in docs_url:
            raise ValidationError(
                f"{path}.docs_url should include docs page for function '{fn_name}'"
            )

        id_parts = indicator_id.split(".")
        if mode == "module":
            if len(id_parts) != 2 or id_parts[0] != module:
                raise ValidationError(
                    f"{path}.id must be '<module>.<function>' when mode is 'module'"
                )
            expected_prefix = f"centaur_technical_indicators::{module}::"
            if "::single::" in function_path or "::bulk::" in function_path:
                raise ValidationError(
                    f"{path}.function_path must not include single/bulk when mode is 'module'"
                )
            if not function_path.startswith(expected_prefix):
                raise ValidationError(
                    f"{path}.function_path must start with '{expected_prefix}'"
                )
        else:
            if len(id_parts) != 3 or id_parts[0] != module or id_parts[1] != mode:
                raise ValidationError(
                    f"{path}.id must be '<module>.<mode>.<function>' when mode is '{mode}'"
                )
            expected_fragment = f"::{mode}::"
            if expected_fragment not in function_path:
                raise ValidationError(
                    f"{path}.function_path must include '{expected_fragment}' for mode '{mode}'"
                )

        if fallible:
            expected_prefix = "centaur_technical_indicators::Result<"
            if not return_type.startswith(expected_prefix):
                raise ValidationError(
                    f"{path}.returns.return_type must start with '{expected_prefix}' when fallible is true"
                )
        elif "Result<" in return_type:
            raise ValidationError(
                f"{path}.returns.return_type must not use Result when fallible is false"
            )

        if is_deprecated and deprecation is None:
            raise ValidationError(f"{path}.deprecation is required when is_deprecated is true")
        if not is_deprecated and deprecation is not None:
            raise ValidationError(
                f"{path}.deprecation must be omitted when is_deprecated is false"
            )
        if deprecation and "replacement_function_path" in deprecation:
            seen_deprecated_replacements.append(
                (path, deprecation["replacement_function_path"])
            )

    for path, replacement_function_path in seen_deprecated_replacements:
        if replacement_function_path not in seen_paths:
            raise ValidationError(
                f"{path}.deprecation.replacement_function_path must reference a known function_path"
            )


def main() -> None:
    schema = json.loads(SCHEMA_PATH.read_text())
    registry = json.loads(REGISTRY_PATH.read_text())
    validate(schema, registry, "registry")
    validate_registry_invariants(registry)
    print("indicator_registry validation passed")


if __name__ == "__main__":
    try:
        main()
    except ValidationError as err:
        print(f"indicator_registry validation failed: {err}", file=sys.stderr)
        sys.exit(1)
