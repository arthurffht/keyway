# AI assistance

Keyway is developed with the help of [Claude Code](https://claude.com/claude-code). This file
explains how, so users, reviewers and distributions can judge the work for themselves.

## What the AI does

- Implements issues in the Rust crates, tests, benchmarks, build files, CI and documentation,
  following the rules in [CLAUDE.md](CLAUDE.md) and [docs/PLAN.md §0](docs/PLAN.md).
- Drafts the GNOME Shell extension (`extension/`). The maintainer reviews every line and
  rewrites anything they cannot explain before it is submitted to extensions.gnome.org
  (authorship mode B, [docs/PLAN.md §31.6.3](docs/PLAN.md)).
- Reviews pull requests on request.

## What people do

- The maintainer decides product direction, reviews every pull request, merges, and publishes
  releases. The maintainer is the accountable author of every merged change.
- Shipped translations and synonym dictionaries are written or reviewed by fluent people.
  Machine drafts are committed only as `#, fuzzy`, which keeps them out of releases.
- Distribution manifests submitted to Flathub are written by hand.

## What the AI never does

Push to `main`, merge pull requests, publish releases, change repository settings or submit
anything to extensions.gnome.org, Flathub, Launchpad, COPR or the AUR without the maintainer's
explicit approval.

## Disclosure in commits

Commits with significant AI-generated content end with the trailer

```text
Assisted-by: Claude Code
```

following the [Fedora AI-assisted contributions policy](https://docs.fedoraproject.org/en-US/council/policy/ai-contribution-policy/).

## The product itself

Keyway contains no AI features and no models. See [docs/adr/0013](docs/adr/0013-no-generative-ai-in-the-product.md).
