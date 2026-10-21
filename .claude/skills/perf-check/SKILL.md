---
name: perf-check
description: Measure Keyway against its performance budgets (docs/performance.md) and compare with main. Use before opening a PR that touches a hot path (ranking, matching, normalization, D-Bus encoding, extension rendering) or when asked about performance.
---

1. Read docs/performance.md and identify which budgets the change can affect.
2. Run `just bench` on the branch and on `main` (use a worktree for `main`); compare
   instruction counts. A regression above 3 % fails the gate.
3. For memory budgets, run the integration test that samples `/proc/<pid>/smaps_rollup`.
4. Never compare wall-clock numbers across different machines or loads; use them only as
   supporting evidence next to instruction counts.
5. Summarize in the PR: a table of budget, `main`, branch and delta, plus the raw command output.
6. If a budget is exceeded, stop and report; do not loosen a budget without an ADR.
