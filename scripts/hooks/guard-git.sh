#!/usr/bin/env bash
# SPDX-FileCopyrightText: Keyway contributors
# SPDX-License-Identifier: GPL-3.0-or-later
# Enforces the CLAUDE.md hard rules that allowlisting git and gh would otherwise open up.
set -euo pipefail
cmd=$(jq -r '.tool_input.command // empty')
deny() { printf '%s\n' "$1" >&2; exit 2; }

# Each rule looks only at its own command segment (up to ; & | or a newline).
push=$(grep -oE 'git[[:space:]]+push[^;&|]*' <<<"$cmd" || true)
if [[ -n "$push" ]]; then
  [[ "$push" =~ (^|[[:space:]:/])(main|master)([[:space:]]|$) ]] && deny "Pushing to main is for the maintainer."
  [[ "$push" != *origin* && "$(git branch --show-current)" == main ]] && deny "Pushing to main is for the maintainer."
fi
commit=$(grep -oE 'git[[:space:]]+commit.*' <<<"$cmd" || true)
if [[ -n "$commit" ]]; then
  [[ "$commit" == *Co-Authored-By* ]] && deny "Use the Assisted-by: Claude Code trailer instead (CLAUDE.md)."
  [[ "$(git branch --show-current)" == main ]] && deny "Commit on a feature branch, not main."
fi
exit 0
