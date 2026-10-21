#!/usr/bin/env bash
# SPDX-FileCopyrightText: Keyway contributors
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Installs CI dependencies inside a distribution container.
# Usage: ci/install-deps.sh <image> <job>
set -euo pipefail

image=$1
job=$2

case "$image" in
    ubuntu*)
        export DEBIAN_FRONTEND=noninteractive
        apt-get update -qq
        common=(ca-certificates git build-essential pkg-config meson ninja-build gettext
            desktop-file-utils appstream libglib2.0-bin libglib2.0-dev)
        case "$job" in
            lint) apt-get install -y -qq --no-install-recommends "${common[@]}" curl rustup reuse shellcheck ;;
            # The distribution's own Rust, which is what packagers build with (1.93 on 26.04).
            test) apt-get install -y -qq --no-install-recommends "${common[@]}" cargo rustc ;;
            *) echo "unknown job: $job" >&2; exit 1 ;;
        esac
        ;;
    fedora*)
        [[ $job == test ]] || { echo "fedora runs only the test job" >&2; exit 1; }
        dnf install -y -q git gcc pkgconf meson ninja-build gettext desktop-file-utils \
            appstream glib2-devel cargo rust
        ;;
    *)
        echo "unsupported image: $image" >&2
        exit 1
        ;;
esac
