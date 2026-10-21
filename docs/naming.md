# Name availability

Record of the name gate in [PLAN.md §2.3](PLAN.md#23-gate--name-availability-check).

- [x] GitHub repositories (`gh search repos keyway --limit 50`, or the public search API)
- [x] extensions.gnome.org search for "keyway"
- [x] Flathub search for "keyway"
- [x] crates.io (`https://crates.io/api/v1/crates/keyway`)
- [x] npm (informational only)
- [ ] Domains such as `keyway.app` and `keyway.dev` (optional)
- [ ] Trademark databases in classes 9 and 42: USPTO, EUIPO, and INPI for Brazil (manual) — **deferred by the maintainer on 2026-10-21 until before the Phase 3 public demo**
- [x] Ubuntu, Debian, Fedora, Arch and AUR package indexes, and the Snap Store

## Results of the automated checks (2026-10-21)

| Channel | Result |
|---|---|
| GitHub account `arthurffht` | Exists |
| Repository `arthurffht/keyway` | Free |
| GitHub repositories named "keyway" | 95 matches, all small. The most visible: `keywaysh/keyway` (GitHub-native secrets management, 12 stars, active in 2026), `HalxDocs/keyway` (SSO library for Go), `Keyway-AI/keyway` (JWT key-rotation tooling) and `TrevorS/keyway` (Bash lock files, 2016). None is a desktop or GNOME project |
| crates.io `keyway` | Free |
| npm `keyway` | Taken by an unrelated 2022 utility library; irrelevant, because Keyway publishes nothing to npm |
| extensions.gnome.org | No extension named Keyway |
| Flathub | No app named Keyway |
| Snap Store | Free |
| Arch Linux (official and AUR) | Free |
| Debian sources | Free |
| Fedora (rawhide) | Free |
| Ubuntu archive | Free |
| Trademark databases (USPTO, EUIPO, INPI) | **Pending: manual search by the maintainer before the Phase 3 public demo** |
| Domains | Optional; not checked |

**Assessment.** "Keyway" is free in every channel the project ships through. The other projects using the word are developer-security tools in a different market. The remaining risk is a registered trademark held by one of them, which the manual trademark search resolves before anything is announced publicly.

If a check fails, use the fallbacks in this order: **Peregrine** (the fastest bird on Earth, with exceptional sight), then **Beckon** (to summon with a gesture). Apply the same check to the fallback.

