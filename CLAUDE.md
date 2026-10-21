# Keyway

Instant search and actions for GNOME 50+ (Wayland only). A thin GNOME Shell extension draws
the bar; a Rust engine (GIO only, D-Bus-activated) does all search, ranking and actions; a
GTK 4 + libadwaita app handles preferences. The full specification is docs/PLAN.md.
Architecture rules live in docs/adr/ — read them before changing structure.

## Layout
- crates/keyway-core       pure Rust: normalization, query parsing, matching, scoring, frecency
- crates/keyway-catalog    build.rs compiles actions/*.toml and locales/*/synonyms.toml
- crates/keyway-calc       fend-core wrapper, locale-aware numbers
- crates/keyway-platform   GIO: apps, providers, backends, capability probing
- crates/keyway-engine     GApplication service: Engine1 + SearchProvider2
- crates/keyway-app        preferences app (Blueprint UI in data/ui)
- crates/keywayctl         CLI: query, bench, doctor, dump-schemas
- crates/keyway-testkit    test harnesses (isolated D-Bus, dbusmock, corpus, Shell e2e)
- extension/               GNOME Shell extension (GJS, ESM). Authorship mode: B (Claude drafts, the maintainer reviews every line; docs/PLAN.md §31.6.3)

## Commands
- just check      formatting, clippy, ESLint, type checks, Blueprint format, REUSE, data validation
- just test       all PR suites except e2e
- just e2e 51     end-to-end Shell suite in a container (also: just e2e 50)
- just bench      deterministic benchmarks compared with main
- keywayctl query "<text>"   inspect ranking with per-source timings

Recipes for components that do not exist yet (e2e, bench, extension lint) are added together
with those components.

## Environment
- Host packages come from the `keyway-dev-env` metapackage (scripts/setup/install-system-deps.sh);
  Rust and cargo tools from scripts/setup/install-user-tools.sh. scripts/setup/uninstall.sh
  removes both. Never install host packages any other way; add them to
  scripts/setup/keyway-dev-env.control instead.
- Debian packaging tools (debhelper, dh-cargo, lintian) run in containers only: dh-cargo
  conflicts with rustup.

## Hard rules
- Never change the live desktop: no gsettings set, dconf write/reset, gnome-extensions
  enable/disable/install on the host. Use `just dev-shell` or `just e2e`.
- Never push to main, merge, release, change repository settings or submit to EGO/Flathub/
  Launchpad/COPR/AUR without the maintainer's explicit approval in chat.
- One event loop (GLib MainContext). No Tokio, no threads without an ADR.
- keyway-engine must not link GTK. keyway-core must not depend on GLib.
- Tests use gio::TestDBus, GSETTINGS_BACKEND=memory, temporary HOME/XDG dirs, dbusmock.
- Minimum platform: GNOME 50, GTK 4.22, libadwaita 1.9, Rust 1.93. No newer APIs without a
  compat path and an ADR.
- Performance budgets in docs/performance.md are tests. Measure before and after.
- Verify every platform API in docs or installed sources before use (docs/PLAN.md §31.6.6).
- Shipped translations must be human-reviewed; AI drafts are committed only as `#, fuzzy`.

## Code style
- Verified APIs only. No defensive error handling for operations that cannot fail.
- Comments explain why, never what. No comments addressed to the reader.
- No speculative abstractions, unused parameters or code "for later".
- Smallest diff that solves the task; match surrounding style; do not reformat unrelated code.
- GNOME vocabulary in UI strings (Dark Style, Night Light, Do Not Disturb), US English,
  header capitalization for titles.
- Every bug fix includes a test that failed before the fix.

## Commits and pull requests
- Conventional Commits with scope: feat(engine): …, fix(extension): …
- End commits that contain significant AI-generated content with:
  Assisted-by: Claude Code
  (this replaces the default Co-Authored-By trailer for this repository)
- PRs: one concern, ~400 changed lines max, PR template filled, command output quoted.

## Definition of Done
just check · just test · just e2e 51 (engine API or extension changes) · just bench (hot paths)
· docs/ADRs/verification ledger updated · maintainer review and merge.
