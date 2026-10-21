---
status: accepted
date: 2026-10-21
decision-makers: arthurffht
---

# The engine links GIO only, never GTK

## Context and Problem Statement

Recorded during the project bootstrap from [docs/PLAN.md §8](../PLAN.md#8-architecture-decisions).
The plan gives the full context for this decision.

## Decision Outcome

The engine links GIO only, never GTK.

### Rationale

*Measured:* GIO-only daemons (`gsd-*`) use 8–14 MiB RSS, while GJS services use 31–35 MiB. Initializing GTK adds cost for nothing the engine needs.

### Consequences

* Changing this decision requires a new ADR that supersedes this one.
