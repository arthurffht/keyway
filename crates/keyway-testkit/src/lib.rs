// SPDX-FileCopyrightText: Keyway contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Test harnesses for Keyway (docs/PLAN.md §26.1). Development only; never shipped.

use std::io;
use std::path::{Path, PathBuf};

/// A private session bus that D-Bus-activates one service, so tests never touch the
/// developer's session.
///
/// Starting it points `DBUS_SESSION_BUS_ADDRESS` of the current process at the private bus,
/// so use at most one per test process.
#[derive(Debug)]
pub struct IsolatedBus {
    bus: gio::TestDBus,
    service_dir: PathBuf,
}

impl IsolatedBus {
    /// Starts the bus with `executable` installed as the activatable service `bus_name`.
    ///
    /// # Errors
    ///
    /// Fails if the temporary service directory or file cannot be written.
    pub fn with_service(bus_name: &str, executable: &Path) -> io::Result<Self> {
        let service_dir = glib::mkdtemp(std::env::temp_dir().join("keyway-test-XXXXXX"))
            .ok_or_else(io::Error::last_os_error)?;
        std::fs::write(
            service_dir.join(format!("{bus_name}.service")),
            format!(
                "[D-BUS Service]\nName={bus_name}\nExec={}\n",
                executable.display()
            ),
        )?;
        let service_dir_str = service_dir
            .to_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "non-UTF-8 temp dir"))?;

        let bus = gio::TestDBus::new(gio::TestDBusFlags::NONE);
        bus.add_service_dir(service_dir_str);
        bus.up();
        Ok(Self { bus, service_dir })
    }
}

impl Drop for IsolatedBus {
    fn drop(&mut self) {
        // Stopping the bus also ends the activated service, which exits when its connection
        // closes.
        self.bus.down();
        let _ = std::fs::remove_dir_all(&self.service_dir);
    }
}
