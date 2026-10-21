# Contributing to Keyway

Thank you for your interest. Keyway is young, so the most useful contributions right now are
bug reports, ideas in [Discussions](https://github.com/arthurffht/keyway/discussions), and —
once the catalog exists — new actions and synonyms.

## Setting up

Follow the *Development* section of the [README](README.md), then run `just doctor` to check
that every tool is present.

## Workflow

1. Open or pick an issue with clear acceptance criteria.
2. Create a branch named `feat/<issue>-<slug>` or `fix/<issue>-<slug>`.
3. Make one change per pull request, at most about 400 changed lines, with tests.
4. Run the Definition of Done commands ([docs/PLAN.md §35.6](docs/PLAN.md#356-definition-of-done))
   and paste their output into the pull request template.
5. The maintainer reviews and squash-merges.

## Standards

- Rust, GJS, Blueprint and Meson conventions: [docs/PLAN.md §28](docs/PLAN.md#28-code-standards).
- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org) with a
  scope, for example `feat(engine): add prefix associations`.
- Every file carries an SPDX header or is covered by `REUSE.toml`; `reuse lint` must pass.
- Architectural changes need an ADR in `docs/adr/` first.
- User-facing text uses US English and GNOME's vocabulary (*Dark Style*, *Night Light*,
  *Do Not Disturb*), with header capitalization for titles.

## Translations

Translations go through gettext. Shipped translations must be written or reviewed by fluent
people; machine drafts are committed only with the `#, fuzzy` flag. See
[docs/PLAN.md §18](docs/PLAN.md#18-internationalization).

## AI assistance

Keyway is developed with the help of Claude Code under the rules in
[AI-ASSISTANCE.md](AI-ASSISTANCE.md). If you use AI tools, disclose it the same way.

## Conduct

Everyone taking part follows the [Code of Conduct](CODE_OF_CONDUCT.md).
