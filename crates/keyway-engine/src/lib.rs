// SPDX-FileCopyrightText: Keyway contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Keyway's search engine, `keyway-engine`: a D-Bus-activated `GApplication` service that
//! links GIO only (docs/adr/0004, docs/adr/0005).
//!
//! This library exposes the names clients need to reach the engine on the session bus.

/// The application ID. Meson sets `KEYWAY_APP_ID` to the `.Devel` ID for development builds,
/// so a development engine can run next to a released one.
pub const APP_ID: &str = match option_env!("KEYWAY_APP_ID") {
    Some(id) => id,
    None => "io.github.arthurffht.Keyway",
};

/// Object path of the `org.gnome.Shell.SearchProvider2` implementation.
pub const SEARCH_PROVIDER_PATH: &str = "/io/github/arthurffht/Keyway/Engine/SearchProvider";

/// Interface name of GNOME Shell's search provider API.
pub const SEARCH_PROVIDER_INTERFACE: &str = "org.gnome.Shell.SearchProvider2";

/// The engine's well-known name on the session bus.
pub fn bus_name() -> String {
    format!("{APP_ID}.Engine")
}
