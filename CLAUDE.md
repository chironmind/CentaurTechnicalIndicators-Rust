# CLAUDE.md

Claude Code notes for this repository. **[`AGENTS.md`](./AGENTS.md) is canonical**
for every standing convention — how work arrives, the pre-PR gates, branch/commit/PR
conventions, change-scope discipline, stop-and-report, and changelog coupling. Read
it before making changes; this file adds only Claude-Code specifics and does not
repeat it.

- **Never guess.** If information is missing or the task is ambiguous, stop and ask
  for input — don't assume.
- **Effort:** if your task is a slice brief, set the session to its `effort` with
  `/effort <level>` before starting.
- **Plan before auto-editing:** for non-trivial work, plan in plan mode before
  auto-applying changes.
- **Read-order:** your brief is self-contained — see `AGENTS.md` → "How work
  arrives." Don't pull in the implementation plan or project spec to execute a
  brief; if you hit a gap, stop and ask.
