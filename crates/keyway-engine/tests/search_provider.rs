// SPDX-FileCopyrightText: Keyway contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;
use std::path::Path;

use glib::variant::ToVariant;
use keyway_engine::{SEARCH_PROVIDER_INTERFACE, SEARCH_PROVIDER_PATH};
use keyway_testkit::IsolatedBus;

// One test per process: the isolated bus replaces this process's session bus.
#[test]
fn engine_is_activated_and_answers_activities_search() {
    let bus_name = keyway_engine::bus_name();
    let engine = Path::new(env!("CARGO_BIN_EXE_keyway-engine"));
    let _bus = IsolatedBus::with_service(&bus_name, engine).expect("isolated bus");
    let connection =
        gio::bus_get_sync(gio::BusType::Session, gio::Cancellable::NONE).expect("session bus");

    let call = |method: &str, params: glib::Variant| {
        connection
            .call_sync(
                Some(&bus_name),
                SEARCH_PROVIDER_PATH,
                SEARCH_PROVIDER_INTERFACE,
                method,
                Some(&params),
                None,
                gio::DBusCallFlags::NONE,
                10_000,
                gio::Cancellable::NONE,
            )
            .unwrap_or_else(|error| panic!("{method}: {error}"))
    };

    let (results,) = call("GetInitialResultSet", (vec!["key"],).to_variant())
        .get::<(Vec<String>,)>()
        .expect("as");
    assert_eq!(results, ["keyway-installed"]);

    let (subset,) = call(
        "GetSubsearchResultSet",
        (results.clone(), vec!["keyway"]).to_variant(),
    )
    .get::<(Vec<String>,)>()
    .expect("as");
    assert_eq!(subset, results);

    let (metas,) = call("GetResultMetas", (results,).to_variant())
        .get::<(Vec<HashMap<String, glib::Variant>>,)>()
        .expect("aa{sv}");
    assert_eq!(metas.len(), 1);
    assert_eq!(metas[0]["id"].str(), Some("keyway-installed"));
    assert_eq!(metas[0]["name"].str(), Some("Keyway"));
    assert!(gio::Icon::deserialize(&metas[0]["icon"]).is_some());

    let (none,) = call("GetInitialResultSet", (vec!["files"],).to_variant())
        .get::<(Vec<String>,)>()
        .expect("as");
    assert_eq!(none, Vec::<String>::new());

    call(
        "ActivateResult",
        ("keyway-installed", vec!["key"], 0_u32).to_variant(),
    );
    call("LaunchSearch", (vec!["key"], 0_u32).to_variant());
    call("XUbuntuCancel", ().to_variant());
}
