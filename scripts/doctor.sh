#!/usr/bin/env bash
# SPDX-FileCopyrightText: Keyway contributors
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Checks that every development tool is installed (docs/PLAN.md §30).
set -uo pipefail

missing=0
check() {
    local name=$1; shift
    if command -v "$name" >/dev/null; then
        printf '  ok       %-22s %s\n' "$name" "$("$@" 2>&1 | head -n1)"
    else
        printf '  MISSING  %s\n' "$name"
        missing=1
    fi
}

echo "Host tools (scripts/setup/install-system-deps.sh):"
check git git --version
check gh gh --version
check meson meson --version
check ninja ninja --version
check just just --version
check jq jq --version
check pkg-config pkg-config --version
check blueprint-compiler blueprint-compiler --version
check xgettext xgettext --version
check glib-compile-schemas glib-compile-schemas --version
check desktop-file-validate desktop-file-validate --version
check appstreamcli appstreamcli --version
check reuse reuse --version
check shellcheck shellcheck --version
check node node --version
check npm npm --version
check podman podman --version
check hyperfine hyperfine --version
check valgrind valgrind --version
check sysprof sysprof --version

echo "Libraries:"
for lib in glib-2.0:2.88 gio-2.0:2.88 gtk4:4.22 libadwaita-1:1.9; do
    name=${lib%%:*} min=${lib##*:}
    if pkg-config --atleast-version="$min" "$name"; then
        printf '  ok       %-22s %s\n' "$name" "$(pkg-config --modversion "$name")"
    else
        printf '  MISSING  %s >= %s\n' "$name" "$min"
        missing=1
    fi
done

echo "Rust (scripts/setup/install-user-tools.sh):"
check rustup rustup --version
check cargo cargo --version
check rustc rustc --version
for tool in nextest deny llvm-cov mutants fuzz bloat; do
    check "cargo-$tool" cargo "$tool" --version
done

if ((missing)); then
    echo "Some tools are missing. See README.md › Development."
    exit 1
fi
echo "All tools present."
