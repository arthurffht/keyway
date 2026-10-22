# SPDX-FileCopyrightText: Keyway contributors
# SPDX-License-Identifier: CC0-1.0
set shell := ["bash", "-euo", "pipefail", "-c"]
build := "_build"

default: check test

doctor:
    scripts/doctor.sh

setup:
    meson setup {{build}} -Dprofile=development --prefix="$HOME/.local"

build:
    meson compile -C {{build}}

check:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets --locked -- -D warnings
    cargo deny check
    reuse lint
    shellcheck build-aux/*.sh scripts/*.sh scripts/*/*.sh ci/*.sh
    meson test -C {{build}} --suite data --print-errorlogs

test:
    cargo nextest run --workspace --locked --no-tests=pass

fuzz target seconds="60":
    cargo +nightly fuzz run --fuzz-dir fuzz {{target}} -- -max_total_time={{seconds}}

cov:
    cargo llvm-cov nextest --workspace --html --no-tests=pass

pot:
    meson compile -C {{build}} keyway-pot
