---
status: accepted
date: 2026-10-21
decision-makers: arthurffht
---

# Reuse existing indexes and providers

## Context and Problem Statement

Recorded during the project bootstrap from [docs/PLAN.md §8](../PLAN.md#8-architecture-decisions).
The plan gives the full context for this decision.

## Decision Outcome

Reuse existing indexes and providers.

### Rationale

LocalSearch already indexes names and content, so a second indexer would only duplicate I/O.

### Consequences

* Changing this decision requires a new ADR that supersedes this one.
