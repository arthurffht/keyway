---
name: new-action
description: Add a Keyway catalog action (setting or command) with synonyms, golden-corpus queries and a round-trip test. Use when asked to add or change an action in actions/.
---

1. Confirm the GSettings schema/key or D-Bus interface exists on GNOME 50 and 51 using
   tests/platform/gnome-*/schemas.json or the installed sources (docs/PLAN.md §31.6.6).
2. Create actions/<domain>/<name>.toml following docs/PLAN.md §13.1. English title in
   header capitalization; GNOME vocabulary; symbolic icon that exists in Adwaita.
3. Add synonyms to locales/en/synonyms.toml and locales/pt_BR/synonyms.toml (natural
   phrasing, not literal translation). Mark pt_BR additions for maintainer review in the PR.
4. Add at least three queries per locale to tests/queries/<locale>.toml.
5. Add or extend the round-trip test (apply → undo → original) in keyway-platform.
6. Run `just test` and quote the catalog, corpus and round-trip results in the PR.
