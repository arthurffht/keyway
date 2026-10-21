#!/usr/bin/env bash
# SPDX-FileCopyrightText: Keyway contributors
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Builds the Rust binaries for Meson and copies them to Meson's outputs. Each output's file
# name is the name of the Cargo binary that produces it.
# Usage: cargo.sh <cargo> <source root> <target dir> <default|development> <output>...
set -euo pipefail

cargo=$1
source_root=$2
target_dir=$3
profile=$4
shift 4

args=(--manifest-path "$source_root/Cargo.toml" --target-dir "$target_dir" --locked)
if [[ $profile == default ]]; then
    args+=(--release)
    build_dir=release
else
    build_dir=debug
fi
for output in "$@"; do
    args+=(--bin "$(basename "$output")")
done

"$cargo" build "${args[@]}"
for output in "$@"; do
    cp "$target_dir/$build_dir/$(basename "$output")" "$output"
done
