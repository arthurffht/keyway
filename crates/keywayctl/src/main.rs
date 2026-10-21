// SPDX-FileCopyrightText: Keyway contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! `keywayctl`: command-line diagnostics for Keyway.

#![allow(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "a command-line tool"
)]

use std::process::ExitCode;
use std::time::Instant;

use glib::variant::ToVariant;
use keyway_engine::{SEARCH_PROVIDER_INTERFACE, SEARCH_PROVIDER_PATH};

const USAGE: &str = "Usage: keywayctl doctor";

/// Long enough for a cold D-Bus activation on a slow machine.
const CALL_TIMEOUT_MS: i32 = 10_000;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.iter().map(String::as_str).collect::<Vec<_>>()[..] {
        ["doctor"] => doctor(),
        ["--help" | "-h"] => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn doctor() -> ExitCode {
    let bus_name = keyway_engine::bus_name();
    println!("Keyway {}, engine {bus_name}", env!("CARGO_PKG_VERSION"));

    let connection = match gio::bus_get_sync(gio::BusType::Session, gio::Cancellable::NONE) {
        Ok(connection) => connection,
        Err(error) => return fail("session bus", &error),
    };
    report_ok("session bus");

    match activatable_names(&connection) {
        Ok(names) if names.contains(&bus_name) => report_ok("engine is installed (activatable)"),
        Ok(_) => {
            println!("  MISSING  engine is not installed: no D-Bus service file for {bus_name}");
            return ExitCode::FAILURE;
        }
        Err(error) => return fail("listing activatable services", &error),
    }

    let started = Instant::now();
    let reply = connection.call_sync(
        Some(&bus_name),
        SEARCH_PROVIDER_PATH,
        SEARCH_PROVIDER_INTERFACE,
        "GetInitialResultSet",
        Some(&(vec!["keyway"],).to_variant()),
        None,
        gio::DBusCallFlags::NONE,
        CALL_TIMEOUT_MS,
        gio::Cancellable::NONE,
    );
    match reply.map(|reply| reply.get::<(Vec<String>,)>()) {
        Ok(Some((results,))) if !results.is_empty() => {
            report_ok(&format!(
                "search provider answered in {} ms",
                started.elapsed().as_millis()
            ));
            ExitCode::SUCCESS
        }
        Ok(_) => {
            println!("  FAILED   search provider returned no result for \"keyway\"");
            ExitCode::FAILURE
        }
        Err(error) => fail("search provider", &error),
    }
}

fn activatable_names(connection: &gio::DBusConnection) -> Result<Vec<String>, glib::Error> {
    let reply = connection.call_sync(
        Some("org.freedesktop.DBus"),
        "/org/freedesktop/DBus",
        "org.freedesktop.DBus",
        "ListActivatableNames",
        None,
        None,
        gio::DBusCallFlags::NONE,
        CALL_TIMEOUT_MS,
        gio::Cancellable::NONE,
    )?;
    Ok(reply
        .get::<(Vec<String>,)>()
        .map(|(names,)| names)
        .unwrap_or_default())
}

fn report_ok(check: &str) {
    println!("  ok       {check}");
}

fn fail(check: &str, error: &glib::Error) -> ExitCode {
    println!("  FAILED   {check}: {error}");
    ExitCode::FAILURE
}
