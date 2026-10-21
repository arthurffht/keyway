---
status: accepted
date: 2026-10-21
decision-makers: arthurffht
---

# D-Bus through GDBus (`gio`), not `zbus`

## Context and Problem Statement

Recorded during the project bootstrap from [docs/PLAN.md §8](../PLAN.md#8-architecture-decisions).
The plan gives the full context for this decision.

## Decision Outcome

D-Bus through GDBus (`gio`), not `zbus`.

### Rationale

One main loop for D-Bus, GSettings and file monitors, and no extra executor threads.

### Consequences

* Changing this decision requires a new ADR that supersedes this one.
