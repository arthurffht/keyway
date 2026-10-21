---
status: accepted
date: 2026-10-21
decision-makers: arthurffht
---

# Native packages first; Flatpak only for the preferences app, later

## Context and Problem Statement

Recorded during the project bootstrap from [docs/PLAN.md §8](../PLAN.md#8-architecture-decisions).
The plan gives the full context for this decision.

## Decision Outcome

Native packages first; Flatpak only for the preferences app, later.

### Rationale

The product depends on a Shell extension and host settings, which the Flatpak sandbox cannot provide.

### Consequences

* Changing this decision requires a new ADR that supersedes this one.
