---
status: accepted
date: 2026-10-21
decision-makers: arthurffht
---

# No database. User configuration lives in GSettings; learned state in one small JSON file written atomically

## Context and Problem Statement

Recorded during the project bootstrap from [docs/PLAN.md §8](../PLAN.md#8-architecture-decisions).
The plan gives the full context for this decision.

## Decision Outcome

No database. User configuration lives in GSettings; learned state in one small JSON file written atomically.

### Rationale

Fewer dependencies, trivially inspectable, and a clipboard that never touches disk.

### Consequences

* Changing this decision requires a new ADR that supersedes this one.
