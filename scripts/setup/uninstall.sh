#!/usr/bin/env bash
# SPDX-FileCopyrightText: Keyway contributors
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Removes the Keyway development environment from this machine. Asks before
# each step. Does not touch the repository, git configuration or SSH keys.
set -euo pipefail

ask() {
    local reply
    read -r -p "$1 [y/N] " reply
    [[ $reply == [yY]* ]]
}

# Before the packages go, while podman still exists.
if command -v podman >/dev/null && ask "Delete all podman images and containers of this user?"; then
    podman system reset --force
fi

if dpkg-query -W -f='${Status}' keyway-dev-env 2>/dev/null | grep -q 'install ok installed'; then
    if ask "Remove the keyway-dev-env metapackage and every package it pulled in?"; then
        sudo apt-get purge -y keyway-dev-env
        sudo apt-get autoremove --purge -y
    fi
fi

if [[ -d $HOME/.rustup || -d $HOME/.cargo ]]; then
    if ask "Delete ~/.rustup and ~/.cargo (Rust toolchains, cargo tools and caches)?"; then
        rm -r -- "$HOME/.rustup" "$HOME/.cargo"
        for rc in "$HOME/.profile" "$HOME/.bashrc"; do
            [[ -f $rc ]] && sed -i '/^# keyway-dev-env: cargo tools$/,+1d' "$rc"
        done
    fi
fi

if [[ -d $HOME/.npm ]] && ask "Delete the npm cache in ~/.npm?"; then
    rm -r -- "$HOME/.npm"
fi

echo "Done."
