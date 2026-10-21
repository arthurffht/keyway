# Architecture overview

Summary of [PLAN.md Part II](PLAN.md#part-ii--architecture). Component documents are added here as components are built.


```mermaid
flowchart LR
    subgraph Shell["gnome-shell (compositor process)"]
        EXT["keyway extension (GJS)<br/>bar · shortcut · Shell-side executors<br/>windows · volume · brightness · clipboard"]
        ACT["Activities search"]
    end
    subgraph Session["User session"]
        ENG["keyway-engine (Rust, GIO only)<br/>D-Bus-activated · exits when idle<br/>index · ranking · catalog · actions · undo"]
        APP["keyway (Rust, GTK 4 + libadwaita)<br/>preferences · onboarding · diagnostics"]
        CTL["keywayctl (CLI)<br/>query · bench · doctor"]
    end
    subgraph Platform["GNOME / freedesktop services"]
        GS["GSettings / dconf"]
        PROV["SearchProvider2 providers<br/>Nautilus (LocalSearch) · Characters · Contacts · Calendar · …"]
        SVC["NetworkManager · UPower.PowerProfiles<br/>SettingsDaemon.Rfkill · SessionManager · ScreenSaver · Settings"]
        GIO["GIO AppInfo · recently-used.xbel"]
    end

    EXT -- "io.github.arthurffht.Keyway.Engine1" --> ENG
    ACT -- "org.gnome.Shell.SearchProvider2" --> ENG
    APP -- "Keyway.Engine1 + GSettings" --> ENG
    CTL -- "Keyway.Engine1" --> ENG
    ENG --> GS & PROV & SVC & GIO
```

| Component | Language | Owns | Never does |
|---|---|---|---|
| Extension | GJS (ESM) | The bar UI, the shortcut, modal grab, rendering, and Shell-side actions: launching apps, focusing windows, volume, brightness, clipboard observation | Indexing, ranking, file I/O, network, blocking calls |
| `keyway-engine` | Rust | Sources, normalization, matching, ranking, the catalog, action backends, undo, state, search provider | Linking GTK, drawing UI, keeping work on a timer |
| `keyway` | Rust | Settings UI, onboarding, diagnostics view | Executing actions (it always goes through the engine) |
| `keywayctl` | Rust | Scripted queries, benchmarks, health checks | Being required at runtime |

