// SPDX-FileCopyrightText: Keyway contributors
// SPDX-License-Identifier: GPL-3.0-or-later

mod application;
mod search_provider;

use gio::prelude::*;

fn main() -> glib::ExitCode {
    application::Engine::new().run()
}
