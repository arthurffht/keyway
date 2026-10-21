# Architecture decision records

Decisions are recorded with the [MADR 4](https://adr.github.io/madr/) template.
Add a new record before changing any architectural rule.

| ADR | Decision |
|---|---|
| [0001](0001-record-decisions-as-madr-files.md) | Record decisions as MADR files |
| [0002](0002-the-overlay-is-a-gnome-shell-extension.md) | The overlay is a GNOME Shell extension |
| [0003](0003-all-search-ranking-and-action-logic-runs-out-of.md) | All search, ranking and action logic runs out of process |
| [0004](0004-the-engine-links-gio-only-never-gtk.md) | The engine links GIO only, never GTK |
| [0005](0005-the-engine-is-a-d-bus-activated-gapplication.md) | The engine is a D-Bus-activated `GApplication` service with an inactivity timeout |
| [0006](0006-d-bus-through-gdbus-gio-not-zbus.md) | D-Bus through GDBus (`gio`), not `zbus` |
| [0007](0007-no-database-user-configuration-lives-in-gsettings.md) | No database. User configuration lives in GSettings; learned state in one small JSON file written atomically |
| [0008](0008-the-catalog-and-synonym-dictionaries-are-compiled.md) | The catalog and synonym dictionaries are compiled into the binary (`build.rs` + `phf`) |
| [0009](0009-reuse-existing-indexes-and-providers.md) | Reuse existing indexes and providers |
| [0010](0010-the-shell-extension-activates-apps-windows-and.md) | The Shell (extension) activates apps, windows and URIs; the engine returns **intents** |
| [0011](0011-meson-is-the-top-level-build-with-cargo-below-it.md) | Meson is the top-level build, with Cargo below it; UI is written in Blueprint |
| [0012](0012-native-packages-first-flatpak-only-for-the.md) | Native packages first; Flatpak only for the preferences app, later |
| [0013](0013-no-generative-ai-in-the-product.md) | No generative AI in the product |
| [0014](0014-english-us-source-strings-gettext-weblate-human.md) | English (US) source strings, gettext, Weblate, human-reviewed translations |
| [0015](0015-minimum-platform-gnome-50-gtk-4-22-libadwaita-1-9.md) | Minimum platform: GNOME 50 (GTK 4.22, libadwaita 1.9) and Rust 1.93 |
| [0016](0016-a-single-main-branch-with-a-compatibility-layer.md) | A single `main` branch with a compatibility layer; an LTS branch only when keeping GNOME 50 costs more than the threshold in §33.5 |
