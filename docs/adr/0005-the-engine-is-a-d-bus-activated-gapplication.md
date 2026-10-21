---
status: accepted
date: 2026-10-21
decision-makers: arthurffht
---

# The engine is a D-Bus-activated `GApplication` service with an inactivity timeout

## Context and Problem Statement

Recorded during the project bootstrap from [docs/PLAN.md §8](../PLAN.md#8-architecture-decisions).
The plan gives the full context for this decision.

## Decision Outcome

The engine is a D-Bus-activated `GApplication` service with an inactivity timeout.

### Rationale

This is the standard GNOME pattern for search providers, and it means zero resident cost when idle.

### Consequences

* Changing this decision requires a new ADR that supersedes this one.
