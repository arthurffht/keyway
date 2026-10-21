# Phase 0 decisions

Decisions taken by the maintainer during the bootstrap ([PLAN.md §37](PLAN.md#37-open-decisions)).

| ID | Decision | Outcome | Date |
|---|---|---|---|
| D-1 | GitHub username | `arthurffht` | 2026-10-21 |
| D-2 | Final name | Keyway; manual trademark search deferred until before the Phase 3 public demo | 2026-10-21 |
| D-3 | Extension authorship mode | **Mode B**: Claude drafts `extension/` under the anti-slop rules; the maintainer reviews every line and rewrites anything they cannot explain before any EGO submission. Until then the extension ships in native packages only | 2026-10-21 |
| D-4 | AI disclosure trailer | `Assisted-by: Claude Code`, replacing the default `Co-Authored-By` trailer | 2026-10-21 |
| D-5 | Default shortcut | `<Super>k` (recommended default, not overridden) | 2026-10-21 |
| D-6 | Visibility and license | Public; GPL-3.0-or-later for code, CC-BY-SA-4.0 for catalog, synonyms, artwork and documentation, CC0-1.0 for metadata, configuration and fixtures | 2026-10-21 |
| D-7 to D-10 | Web search, assistant presets, fuzzy matcher, hosting | Open; recommended defaults apply until decided | — |

## Deviations from the plan made during the bootstrap

- The project folder is `~/Projects/Keyway` (created by the maintainer) instead of `~/Projects/keyway`.
- Host tools are installed through the `keyway-dev-env` metapackage (`scripts/setup/`) so the
  environment can be removed cleanly. `debhelper`, `dh-cargo`, `devscripts` and `lintian` are
  left out of the host: `dh-cargo` conflicts with `rustup`, so packages are built in containers.
- Crate versions in `Cargo.toml` were updated to the current releases (verification ledger).
- The branch ruleset requires only the CI checks that exist today; `extension`, `e2e` and
  `bench` are added to it when their jobs land.
