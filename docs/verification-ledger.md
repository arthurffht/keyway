# Verification ledger

Every platform fact that code depends on is confirmed here first ([PLAN.md §0, rule 3](PLAN.md#0-operating-rules-for-automated-work)). Mark an item ☑ with its evidence when it is verified.


**Confirmed** (source or *measured*):

| Fact | Evidence |
|---|---|
| GNOME Shell 51.0, GTK 4.24.1, libadwaita 1.10.0, GLib 2.90.0, GJS 1.90.0, gettext 1.0 on Ubuntu 26.10 | *Measured* |
| `gnome-shell` 51 options `--headless`, `--virtual-monitor`, `--devkit`, `--profile`, `--debug-control`, `--force-animations` | *Measured* (`--help`) |
| `Main.brightnessManager`, with `globalScale` and `scales`, drives the brightness slider | *Measured* (Shell 51 sources) |
| `getMixerControl()` is exported by `ui/status/volume.js` | *Measured* |
| Dark Style uses `color-scheme`; Do Not Disturb uses `show-banners`; Night Light uses `night-light-enabled` | *Measured* |
| PowerProfiles name and path; Rfkill name, path and airplane-mode properties | *Measured* |
| Search providers are read from system data directories only; `.ini` keys; `org.gnome.desktop.search-providers` keys | *Measured* (`remoteSearch.js`, `fileUtils.js`) |
| `org.gnome.Shell.Eval` requires unsafe mode; `org.gnome.Shell.Extensions` state codes | *Measured* |
| `org.gnome.Shell.Brightness` has no set-level method | *Measured* |
| Ubuntu's `SearchProvider2` includes `XUbuntuCancel` | *Measured* |
| `PopupMenu.Switch`, `Slider`, `Main.pushModal`, `Main.popModal`, `Main.activateWindow` exports | *Measured* |
| `St.Clipboard.get_mimetypes`, `get_text`; `St.ReducedMotion` | *Measured* (`St-51.typelib`) |
| `<Super>space` is bound to `switch-input-source`; `<Super>k` is free | *Measured* |
| GIO daemons use 8–14 MiB RSS; GJS services use 31–35 MiB | *Measured* |
| Ubuntu 26.10 archive versions (§30.1) | *Measured* (`apt-cache policy`) |
| Ubuntu 26.04 ships GNOME 50 and Rust 1.93.1 | linuxiac; packages.ubuntu.com |
| EGO's AI policy; EGO-M-004 | linuxiac; EGO review pages |
| Flathub disclosure policy (October 2026) | GamingOnLinux |
| `xgettext` supports Rust since 0.24 | GNU Savannah |
| `iai-callgrind` was renamed Gungraun | rust-lang/rust PR 150080 |
| libadwaita-rs 0.9.2; `v1_9` feature exists | docs.rs; Debian changelog |
| `claude-code-action@v1`, `/install-github-app`, `claude_code_oauth_token` | Claude Code docs; action setup docs |
| GitHub account `arthurffht` exists; `arthurffht/keyway` is free; "keyway" is free on crates.io, EGO, Flathub, Snap, Arch/AUR, Debian, Fedora and Ubuntu | *Measured* through the public APIs on 2026-10-21 (§2.3) |

**To verify** (status ☐):

- ☑ gtk4 0.11 feature name for GTK 4.22 — `v4_22` (see below)
- ☐ `Main.brightnessManager` on GNOME 50
- ☐ Volume sink API used through `getMixerControl()`
- ☐ `app.launch_action` signature; `global.create_app_launch_context`
- ☐ Layout group for the bar (`ui/layout.js`)
- ☐ `Main.osdWindowManager` method signature
- ☐ `global.context.unsafe_mode` writable from a test extension
- ☐ Nautilus and Characters provider bus names and paths
- ☐ `SearchProvider2` `clipboardText` meta key
- ☐ `org.gnome.Settings` `launch-panel` action parameter
- ☐ PowerProfiles `ActiveProfile`; the Rfkill Bluetooth property; NetworkManager polkit behaviour
- ☐ SessionManager and logind method semantics
- ☐ libadwaita 1.9 CSS variable names
- ☐ GLib version shipped with GNOME 50; Meson and Blueprint versions on Ubuntu 26.04
- ☐ dconf profile isolation via `DCONF_PROFILE`
- ☐ Whether the GNOME OS nightly image can run a headless Shell in CI
- ☑ `blueprint-compiler format --fix` — `-f, --fix` exists in blueprint-compiler 0.20.4 (`format --help`, 2026-10-21)
- ☐ ESLint configuration recommended on gjs.guide
- ☐ Claude Code permission rule syntax
- ☐ Web and assistant URL templates
- ☐ `fend-core` exchange-rate API (after 1.0)
- ☐ Debian GR 2026 outcome
- ☐ `gnome-extensions pack` schema handling for EGO

## Verified during the bootstrap (2026-10-21)

| Fact | Evidence |
|---|---|
| Current crate versions: `glib`/`gio` 0.22.10, `gtk4` 0.11.5, `libadwaita` 0.9.2, `nucleo-matcher` 0.3.1, `phf` 0.14.0, `fend-core` 1.5.9, `quick-xml` 0.42.0, `thiserror` 2.0.21, `gettext-rs` 0.8.0, `toml` 1.1.8. The plan's `phf` 0.11, `quick-xml` 0.37 and `gettext-rs` 0.7 were outdated | crates.io API |
| ☑ gtk4 0.11 feature for GTK 4.22 is `v4_22` | crates.io API, `gtk4` 0.11.5 features |
| `gettext-rs` 0.8 has the `gettext-system` feature; `phf` 0.14 has `macros` | crates.io API |
| `glib`, `gio` and `gtk4` 0.22/0.11 declare `rust-version = 1.92`, below the 1.93 MSRV | crates.io API |
| Latest stable Rust at bootstrap is 1.99.1 (2026-10-13) | static.rust-lang.org channel manifest |
| The Ubuntu `dh-cargo` package depends on `cargo`, which conflicts with `rustup`; Debian packaging tools therefore run in containers | `apt-get install -s` on Ubuntu 26.10 |
