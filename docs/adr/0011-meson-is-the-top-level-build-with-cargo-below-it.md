---
status: accepted
date: 2026-10-21
decision-makers: arthurffht
---

# Meson is the top-level build, with Cargo below it; UI is written in Blueprint

## Context and Problem Statement

Recorded during the project bootstrap from [docs/PLAN.md §8](../PLAN.md#8-architecture-decisions).
The plan gives the full context for this decision.

## Decision Outcome

Meson is the top-level build, with Cargo below it; UI is written in Blueprint.

### Rationale

This is how GNOME's Rust apps are built. Meson installs schemas, desktop files, D-Bus services, icons, translations and the extension.

### Consequences

* Changing this decision requires a new ADR that supersedes this one.
