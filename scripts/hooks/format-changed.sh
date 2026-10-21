#!/usr/bin/env bash
# SPDX-FileCopyrightText: Keyway contributors
# SPDX-License-Identifier: GPL-3.0-or-later
set -euo pipefail
file=$(jq -r '.tool_input.file_path // empty')
[[ -z "$file" ]] && exit 0
case "$file" in
  *.rs)  rustfmt --edition 2024 "$file" ;;
  *.blp) blueprint-compiler format --fix "$file" >/dev/null ;;
  "$CLAUDE_PROJECT_DIR"/extension/*.js) npx --prefix "$CLAUDE_PROJECT_DIR/extension" eslint --fix "$file" ;;
esac
