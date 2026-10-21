#!/usr/bin/env bash
# SPDX-FileCopyrightText: Keyway contributors
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Installs the per-user Rust toolchain and cargo tools (docs/PLAN.md §30.2).
# Everything lands in ~/.rustup and ~/.cargo; nothing needs root.
set -euo pipefail

repo=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
channel=$(awk -F'"' '/^channel/ {print $2}' "$repo/rust-toolchain.toml")

rustup toolchain install "$channel" --profile minimal --component rustfmt,clippy,llvm-tools
rustup toolchain install 1.93 --profile minimal
rustup default "$channel"

cargo install --locked cargo-nextest cargo-deny cargo-llvm-cov cargo-mutants cargo-fuzz cargo-bloat

# Ubuntu's rustup installs its proxies in /usr/bin but leaves ~/.cargo/bin, where cargo
# install puts binaries, off PATH. uninstall.sh removes these marked lines again.
marker='# keyway-dev-env: cargo tools'
# shellcheck disable=SC2016 # expanded later, by the shell that reads the rc file
line='case ":$PATH:" in *":$HOME/.cargo/bin:"*) ;; *) PATH="$HOME/.cargo/bin:$PATH" ;; esac'
for rc in "$HOME/.profile" "$HOME/.bashrc"; do
    if ! grep -qxF "$marker" "$rc" 2>/dev/null; then
        printf '\n%s\n%s\n' "$marker" "$line" >> "$rc"
    fi
done

echo
echo "Rust $channel (development) and 1.93 (MSRV) installed with the cargo tools."
echo "Open a new terminal (or log in again) so ~/.cargo/bin is on PATH."
