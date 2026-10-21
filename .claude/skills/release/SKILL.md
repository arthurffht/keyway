---
name: release
description: Prepare a Keyway release up to a draft GitHub release, following docs/release-checklist.md. Use when the maintainer asks to prepare a release or a release candidate. Publishing is always done by the maintainer.
---

1. Confirm with the maintainer the version number (semantic versioning) and the scope.
2. Walk through docs/release-checklist.md and report the status of every item, with evidence.
3. Update CHANGELOG.md and the AppStream `<releases>` entry; bump versions in Cargo.toml,
   meson.build and extension/metadata.json (`version-name`) in one PR.
4. After the maintainer merges, the maintainer creates and signs the tag. The `release.yml`
   workflow builds a **draft** release.
5. Never publish the release, upload to extensions.gnome.org, or update PPA, COPR or AUR.
   List those steps for the maintainer instead.
