// SPDX-FileCopyrightText: Keyway contributors
// SPDX-License-Identifier: GPL-3.0-or-later

#![no_main]

use keyway_core::query::{DecimalSeparator, parse};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: (bool, &str)| {
    let (comma, text) = input;
    let decimal = if comma { DecimalSeparator::Comma } else { DecimalSeparator::Point };
    let query = parse(text, decimal);
    assert!(text.contains(query.text));
});
