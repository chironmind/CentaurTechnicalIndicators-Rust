#!/usr/bin/env bash
set -euo pipefail

if [[ -z "${GITHUB_BASE_REF:-}" ]]; then
  echo "No base ref detected; skipping policy check."
  exit 0
fi

echo "Running AI policy checks against base branch: ${GITHUB_BASE_REF}"
base_ref="origin/${GITHUB_BASE_REF}"
if ! git rev-parse --verify "${base_ref}" >/dev/null 2>&1; then
  git fetch --no-tags --depth=1 origin "${GITHUB_BASE_REF}"
fi

changed_files="$(git diff --name-only "${base_ref}...HEAD")"
echo "Changed files:"
echo "${changed_files}"

if echo "${changed_files}" | rg -q '^src/.*\.rs$'; then
  if ! echo "${changed_files}" | rg -q '^CHANGELOG\.md$'; then
    echo "Policy check failed: Rust source changes require a CHANGELOG.md update."
    echo "Add a user-facing changelog entry or document why no user-facing change exists."
    exit 1
  fi
fi

pub_api_diff="$(git diff --unified=0 "${base_ref}...HEAD" -- src/*.rs | rg -n '^[+-][[:space:]]*pub[[:space:]]+(fn|struct|enum|trait|type|mod|use|const|static)\b' || true)"
if [[ -n "${pub_api_diff}" ]]; then
  pr_body=""
  if [[ -n "${GITHUB_EVENT_PATH:-}" ]] && [[ -f "${GITHUB_EVENT_PATH}" ]] && command -v jq >/dev/null 2>&1; then
    pr_body="$(jq -r '.pull_request.body // ""' "${GITHUB_EVENT_PATH}")"
  fi

  if ! echo "${pr_body}" | rg -qi '(^|[[:space:]])compatibility([[:space:]]|:|$)'; then
    echo "Policy check failed: public API changes detected, but PR body has no Compatibility section."
    echo "Include a Compatibility section describing user-facing API/error behavior impact."
    echo "Detected public API diff lines:"
    echo "${pub_api_diff}"
    exit 1
  fi
fi

echo "AI policy checks passed."
