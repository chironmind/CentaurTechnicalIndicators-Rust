#!/usr/bin/env python3
"""Generate docs/api_index.json and docs/api_index.md for public crate functions."""

from __future__ import annotations

import json
import re
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
LIB_RS = REPO_ROOT / "src" / "lib.rs"
OUTPUT_JSON = REPO_ROOT / "docs" / "api_index.json"
OUTPUT_MD = REPO_ROOT / "docs" / "api_index.md"

PUB_MOD_RE = re.compile(r"^pub\s+mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;")
MOD_RE = re.compile(r"^(pub\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{")
PUB_FN_RE = re.compile(r"\bpub\s+fn\s+([A-Za-z_][A-Za-z0-9_]*)\b")
DEPRECATED_RE = re.compile(r"^\s*#\[deprecated\((.*)\)\]\s*$")
NOTE_RE = re.compile(r'note\s*=\s*"([^"]*)"')
SINCE_RE = re.compile(r'since\s*=\s*"([^"]*)"')


def enumerate_public_modules() -> list[str]:
    modules = []
    for line in LIB_RS.read_text(encoding="utf-8").splitlines():
        m = PUB_MOD_RE.match(line.strip())
        if m:
            modules.append(m.group(1))
    return modules


def classify_return(return_type: str) -> str:
    stripped = return_type.strip()
    if stripped.startswith("crate::Result<") or stripped.startswith("Result<"):
        return "Result<T>"
    return "plain"


def parse_module_file(module_name: str) -> list[dict[str, object]]:
    module_path = REPO_ROOT / "src" / f"{module_name}.rs"
    lines = module_path.read_text(encoding="utf-8").splitlines()

    functions: list[dict[str, object]] = []
    depth = 0
    module_stack: list[dict[str, object]] = []

    collecting_signature = False
    signature_lines: list[str] = []
    current_fn_name = ""
    pending_deprecated: dict[str, str] | None = None

    def current_context() -> tuple[str, bool]:
        if not module_stack:
            return "top-level", True
        public = True
        names: list[str] = []
        for item in module_stack:
            names.append(item["name"])
            public = public and bool(item["public"])
        if "single" in names:
            return "single", public
        if "bulk" in names:
            return "bulk", public
        return "top-level", public

    for line in lines:
        stripped = line.strip()

        dep_match = DEPRECATED_RE.match(stripped)
        if dep_match:
            args = dep_match.group(1)
            note = NOTE_RE.search(args)
            since = SINCE_RE.search(args)
            pending_deprecated = {
                "since": since.group(1) if since else "",
                "note": note.group(1) if note else "",
                "raw": stripped,
            }

        mod_match = MOD_RE.match(stripped)
        if mod_match:
            is_pub = bool(mod_match.group(1))
            name = mod_match.group(2)
            module_stack.append({"name": name, "public": is_pub, "depth": depth + 1})

        if collecting_signature:
            signature_lines.append(stripped)
            if "{" in stripped:
                full_signature = " ".join(part for part in signature_lines if part)
                full_signature = re.sub(r"\s+", " ", full_signature).rstrip()
                full_signature = full_signature[:-1].rstrip() if full_signature.endswith("{") else full_signature

                return_type = "()"
                rt_match = re.search(r"->\s*(.+)$", full_signature)
                if rt_match:
                    return_type = rt_match.group(1).strip()

                submodule, public_context = current_context()
                if public_context:
                    functions.append(
                        {
                            "module": module_name,
                            "submodule": submodule,
                            "name": current_fn_name,
                            "full_signature": full_signature,
                            "return_type": return_type,
                            "return_kind": classify_return(return_type),
                            "deprecated": pending_deprecated,
                        }
                    )
                collecting_signature = False
                signature_lines = []
                current_fn_name = ""
                pending_deprecated = None
        else:
            fn_match = PUB_FN_RE.search(stripped)
            if fn_match:
                collecting_signature = True
                current_fn_name = fn_match.group(1)
                signature_lines = [stripped]
                if "{" in stripped:
                    # single-line signature
                    full_signature = stripped[:-1].rstrip() if stripped.endswith("{") else stripped
                    return_type = "()"
                    rt_match = re.search(r"->\s*(.+)$", full_signature)
                    if rt_match:
                        return_type = rt_match.group(1).strip()
                    submodule, public_context = current_context()
                    if public_context:
                        functions.append(
                            {
                                "module": module_name,
                                "submodule": submodule,
                                "name": current_fn_name,
                                "full_signature": full_signature,
                                "return_type": return_type,
                                "return_kind": classify_return(return_type),
                                "deprecated": pending_deprecated,
                            }
                        )
                    collecting_signature = False
                    signature_lines = []
                    current_fn_name = ""
                    pending_deprecated = None
            elif stripped and not stripped.startswith("#") and pending_deprecated:
                pending_deprecated = None

        depth += line.count("{") - line.count("}")
        while module_stack and module_stack[-1]["depth"] > depth:
            module_stack.pop()

    functions.sort(key=lambda x: (x["module"], x["submodule"], x["name"]))
    return functions


def build_markdown(modules: list[str], functions: list[dict[str, object]]) -> str:
    lines = [
        "# API Index",
        "",
        "This file is generated by `scripts/generate_api_index.py`.",
        "It enumerates every public function exported by the crate.",
        "",
        f"- Public modules from `src/lib.rs`: {', '.join(f'`{m}`' for m in modules)}",
        f"- Total public functions: **{len(functions)}**",
        "",
        "## Functions",
        "",
        "| Module | Submodule | Function | Return kind | Return type | Deprecated |",
        "| --- | --- | --- | --- | --- | --- |",
    ]

    for fn in functions:
        dep = ""
        if fn["deprecated"]:
            dep_data = fn["deprecated"]
            parts = []
            if dep_data.get("since"):
                parts.append(f"since={dep_data['since']}")
            if dep_data.get("note"):
                parts.append(dep_data["note"])
            dep = " ; ".join(parts) if parts else str(dep_data)

        signature = str(fn["full_signature"]).replace("|", "\\|")
        lines.append(
            "| "
            + " | ".join(
                [
                    str(fn["module"]),
                    str(fn["submodule"]),
                    f"`{fn['name']}`<br/><code>{signature}</code>",
                    str(fn["return_kind"]),
                    f"`{fn['return_type']}`",
                    dep,
                ]
            )
            + " |"
        )

    lines.append("")
    return "\n".join(lines)


def main() -> None:
    modules = enumerate_public_modules()
    all_functions: list[dict[str, object]] = []
    for module in modules:
        all_functions.extend(parse_module_file(module))

    all_functions.sort(key=lambda x: (x["module"], x["submodule"], x["name"], x["full_signature"]))

    payload = {
        "public_modules": modules,
        "function_count": len(all_functions),
        "functions": all_functions,
    }

    OUTPUT_JSON.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    OUTPUT_MD.write_text(build_markdown(modules, all_functions), encoding="utf-8")


if __name__ == "__main__":
    main()
