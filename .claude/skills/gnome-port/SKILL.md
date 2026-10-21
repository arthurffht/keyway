---
name: gnome-port
description: Run the per-cycle GNOME porting runbook (docs/PLAN.md §33.4) for a new GNOME version. Use when a GNOME alpha, beta or RC is out, or when the canary workflow fails.
---

1. Read the gjs.guide porting guide for the target version and list every change that
   touches APIs used in extension/ (grep imports and calls).
2. Run `just e2e <version>` against the matching container or the canary image.
3. Write docs/porting/gnome-<version>.md: changes, impact, required compat entries, tests.
4. Open one issue per required change, labelled gnome-<version>.
5. Never add the version to shell-version before the release candidate (EGO-M-004).
6. In authorship mode B, draft the extension/ changes under the anti-slop rules and list every
   changed Shell API in the PR so the maintainer can review each line.
