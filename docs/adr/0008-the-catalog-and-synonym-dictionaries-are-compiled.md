---
status: accepted
date: 2026-10-21
decision-makers: arthurffht
---

# The catalog and synonym dictionaries are compiled into the binary (`build.rs` + `phf`)

## Context and Problem Statement

Recorded during the project bootstrap from [docs/PLAN.md §8](../PLAN.md#8-architecture-decisions).
The plan gives the full context for this decision.

## Decision Outcome

The catalog and synonym dictionaries are compiled into the binary (`build.rs` + `phf`).

### Rationale

Startup does no parsing, and invalid data fails the build instead of reaching users.

### Consequences

* Changing this decision requires a new ADR that supersedes this one.
