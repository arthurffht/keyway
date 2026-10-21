#!/usr/bin/env bash
# SPDX-FileCopyrightText: Keyway contributors
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Installs the host packages for Keyway development as one metapackage, so the
# whole set can be removed again with scripts/setup/uninstall.sh.
set -euo pipefail

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

mkdir -p "$work/pkg/DEBIAN"
cp "$here/keyway-dev-env.control" "$work/pkg/DEBIAN/control"
version=$(awk '/^Version:/ {print $2}' "$here/keyway-dev-env.control")
deb="$work/keyway-dev-env_${version}_all.deb"
dpkg-deb --root-owner-group --build "$work/pkg" "$deb" >/dev/null

# apt reads local .deb files as the unprivileged _apt user.
chmod 755 "$work"
chmod 644 "$deb"

sudo apt-get update
# Recommends are skipped on purpose: devscripts-style recommends pull in an MTA
# and dozens of unrelated tools. podman's useful recommends are listed in the
# control file explicitly.
sudo apt-get install --no-install-recommends -y "$deb"

echo
echo "keyway-dev-env installed. Next: scripts/setup/install-user-tools.sh"
