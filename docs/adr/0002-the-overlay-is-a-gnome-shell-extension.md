---
status: accepted
date: 2026-10-21
decision-makers: arthurffht
---

# The overlay is a GNOME Shell extension

## Context and Problem Statement

Recorded during the project bootstrap from [docs/PLAN.md §8](../PLAN.md#8-architecture-decisions).
The plan gives the full context for this decision.

## Decision Outcome

The overlay is a GNOME Shell extension.

### Rationale

Mutter has no `wlr-layer-shell` and apps cannot position windows on Wayland. The GlobalShortcuts portal (GNOME 48+) solves shortcuts but not placement.

### Consequences

* Changing this decision requires a new ADR that supersedes this one.
