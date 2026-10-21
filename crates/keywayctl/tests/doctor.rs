// SPDX-FileCopyrightText: Keyway contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::Path;
use std::process::Command;

use keyway_testkit::IsolatedBus;

fn keywayctl() -> Command {
    Command::new(env!("CARGO_BIN_EXE_keywayctl"))
}

// One test per process: the isolated bus replaces this process's session bus, and the
// keywayctl child inherits it.
#[test]
fn doctor_reports_an_installed_engine() {
    let keywayctl_path = Path::new(env!("CARGO_BIN_EXE_keywayctl"));
    // Cargo builds every workspace binary into the same directory for `--workspace` runs.
    let engine = keywayctl_path.with_file_name("keyway-engine");
    assert!(
        engine.exists(),
        "build keyway-engine first: cargo test --workspace"
    );

    let _bus =
        IsolatedBus::with_service(&keyway_engine::bus_name(), &engine).expect("isolated bus");
    let output = keywayctl().arg("doctor").output().expect("run keywayctl");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success(), "{stdout}");
    assert!(stdout.contains("ok       engine is installed"), "{stdout}");
    assert!(
        stdout.contains("ok       search provider answered"),
        "{stdout}"
    );
}

#[test]
fn unknown_arguments_print_usage() {
    let output = keywayctl()
        .arg("frobnicate")
        .output()
        .expect("run keywayctl");
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Usage: keywayctl doctor"));
}
