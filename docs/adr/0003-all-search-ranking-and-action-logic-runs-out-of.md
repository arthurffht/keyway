---
status: accepted
date: 2026-10-21
decision-makers: arthurffht
---

# All search, ranking and action logic runs out of process

## Context and Problem Statement

Recorded during the project bootstrap from [docs/PLAN.md §8](../PLAN.md#8-architecture-decisions).
The plan gives the full context for this decision.

## Decision Outcome

All search, ranking and action logic runs out of process.

### Rationale

Extension code runs on the compositor's main loop, where blocking work drops frames. A separate process also isolates crashes and makes the engine testable.

### Consequences

* Changing this decision requires a new ADR that supersedes this one.
