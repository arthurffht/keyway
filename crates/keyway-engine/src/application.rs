// SPDX-FileCopyrightText: Keyway contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use gio::prelude::*;

/// Seconds the engine stays alive after its last request; matches the default of the
/// `engine-idle-timeout` GSettings key, which the engine does not read yet.
const INACTIVITY_TIMEOUT_SECONDS: u32 = 600;

mod imp {
    use std::cell::RefCell;

    use gio::subclass::prelude::*;

    use crate::search_provider;

    #[derive(Debug, Default)]
    pub(crate) struct Engine {
        search_provider: RefCell<Option<gio::RegistrationId>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Engine {
        const NAME: &'static str = "KeywayEngine";
        type Type = super::Engine;
        type ParentType = gio::Application;
    }

    impl ObjectImpl for Engine {}

    impl ApplicationImpl for Engine {
        // GApplication hands over its bus connection here, before the name is acquired, so
        // the provider is reachable as soon as D-Bus activation completes.
        fn dbus_register(
            &self,
            connection: &gio::DBusConnection,
            object_path: &str,
        ) -> Result<(), glib::Error> {
            self.parent_dbus_register(connection, object_path)?;
            let id = search_provider::register(&self.obj(), connection)?;
            self.search_provider.replace(Some(id));
            Ok(())
        }

        fn dbus_unregister(&self, connection: &gio::DBusConnection, object_path: &str) {
            if let Some(id) = self.search_provider.take() {
                // Fails only for an unknown ID, and this one came from a successful registration.
                let _ = connection.unregister_object(id);
            }
            self.parent_dbus_unregister(connection, object_path);
        }
    }
}

glib::wrapper! {
    pub(crate) struct Engine(ObjectSubclass<imp::Engine>)
        @extends gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Engine {
    pub(crate) fn new() -> Self {
        let engine: Self = glib::Object::builder()
            .property("application-id", keyway_engine::bus_name())
            .property("flags", gio::ApplicationFlags::IS_SERVICE)
            .build();
        engine.set_inactivity_timeout(INACTIVITY_TIMEOUT_SECONDS * 1000);
        engine
    }
}
