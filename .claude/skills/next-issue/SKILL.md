---
name: next-issue
description: Pick the next Keyway issue in dependency order and take it to an open, reviewed, evidence-quoting PR without the maintainer's input. Use when asked to continue, keep going, or work through the roadmap.
---

Stops at an open PR. Merging, pushing to `main` and everything in "Maintainer-only" stay with
the maintainer (CLAUDE.md hard rules).

## 1. Choose the issue

1. `git fetch`, `gh pr list`, `gh issue list --state open`. Finish or unblock open PRs first:
   a `<ci-monitor-event>` is handled before new work.
2. Take the first open issue in this order whose dependencies are done or have an open PR
   (Phase 1, then 2, then 3; docs/PLAN.md §35.2–35.4):
   `4 → 5 → 6 → 7 → 8 → 9 → 10 → 11 → 12 → 13 → 14 → 15 → 16 → 17 → 18 → 19 → 2 → 20 → 21 → 22 → 23 → 24`.
   #6 precedes #7 because scoring needs the matcher decision (D-9, ADR-0017).
3. Skip any issue labelled `blocked`, or whose body says it needs something not yet merged
   (#27 needs the preferences app).
4. Read the issue, its PLAN section, the ADRs it touches (`docs/adr/`), and the surrounding code.

## 2. Branch and stacking

- Branch `feat/<issue>-<slug>` (`fix/` for bugs).
- Base: `main` if its dependencies are merged; otherwise the open PR branch it depends on, and
  open the PR against that branch. Say "Stacked on #N" in the PR body.
- Never push to `main`; never force-push; never merge.

## 3. Implement

- Verify every platform API against installed sources or docs before using it (PLAN §31.6.6);
  record new facts in `docs/verification-ledger.md`.
- Add dependencies only to `[workspace.dependencies]`, versions checked with `cargo search`.
- A new thread, a new runtime or a minimum-platform bump needs an ADR first
  (`docs/adr/`, next number in sequence).
- Every acceptance line in the issue becomes a test that fails without the change.
- `extension/` is authorship mode B: keep it within the 1,200-line budget, and put "needs
  line-by-line maintainer review" in the PR.

## 4. Compare before opening the PR

- `just check`, `just test`; `just e2e 51` for engine API or extension changes.
- Hot path (matching, normalization, scoring, D-Bus encoding, rendering): run the
  `perf-check` skill and put the budget / `main` / branch / delta table in the PR.
- Ranking or synonyms: run the golden corpus and report top-1 accuracy before and after.
- Matcher or library choices: benchmark the candidates side by side and record the numbers in
  the ADR the issue names.
- Run the `slop-reviewer` and `security-reviewer` agents on the diff; add `gnome-reviewer` for
  UI or extension changes. Fix findings before opening the PR.

## 5. Open the PR

- Conventional Commit with scope; trailer `Assisted-by: Claude Code` (not Co-Authored-By);
  `Closes #N`.
- Fill `.github/PULL_REQUEST_TEMPLATE.md` with real command output; leave a checklist item
  unchecked if it was not done, and say why.
- Bind the PR (`get_status`, `bind_pr`) and turn on Auto-fix. Do not poll CI.
- Keep a PR near 400 changed lines; split the issue into stacked PRs otherwise.

## 6. Continue

After the PR is open, go back to step 1 with the new branch as a possible base. Stop and report
instead of continuing when:

- a performance budget fails (never loosen one without an ADR);
- an acceptance criterion cannot be met or the plan contradicts the code;
- the next issue is maintainer-only;
- three stacked PRs are already waiting for review.

## Maintainer-only

#3 (GitHub App and OAuth secret), #25 (demo and the Phase 3 gate), every release, EGO / Flathub
/ Launchpad / COPR / AUR submission, repository settings, translations beyond `#, fuzzy` drafts,
and decisions marked GATE in the plan.
