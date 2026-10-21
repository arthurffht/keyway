# Performance budgets

These budgets are tests, not goals ([PLAN.md §25](PLAN.md#25-performance-engineering)). Baselines were measured on the reference machine on 2026-10-21.


Baselines were *measured* on the reference machine on 2026-10-21. GIO-only GNOME daemons (`gsd-*`) use 8–14 MiB RSS and 0.8–2.6 MiB PSS; GJS-based Shell services use 31–35 MiB RSS.

| Metric | Budget | How it is measured | Enforced by |
|---|---|---|---|
| Engine RSS after 1,000 queries | ≤ 16 MiB | `/proc/<pid>/smaps_rollup` | Integration test (fails above budget) |
| Engine private dirty memory | ≤ 4 MiB | `Private_Dirty` in `smaps_rollup` | Integration test |
| Engine resident cost while idle | 0 (process exited) | `pgrep` after the timeout | Integration test with a short timeout |
| Cold activation → first `Search` reply | p95 ≤ 40 ms in CI, ≤ 25 ms on the reference machine | `hyperfine` with `keywayctl` | Nightly |
| Local `Search()` (2,000 indexed items) | p50 ≤ 0.5 ms, p95 ≤ 2 ms | Engine histograms; `keywayctl bench` | Nightly + release report |
| Ranking hot path | Instruction count within ±3 % of `main` | `gungraun` (Callgrind) | **Every PR** (deterministic, so it does not flake) |
| First external batch | p95 ≤ 120 ms with healthy providers | Diagnostics histograms | Nightly |
| Shortcut → bar visible | ≤ 1 frame | `gnome-shell --profile` capture in Sysprof | Release checklist |
| Typing | Zero dropped frames during a 30-second scripted session | Sysprof frame timings | Release checklist |
| Extension memory growth in the Shell | ≤ 3 MiB after 1,000 queries, with no growth afterwards | Shell `smaps_rollup` in e2e | Nightly e2e |
| CPU while the bar is closed | 0 % (no timers) | Wakeup count over 10 minutes | Nightly |
| Engine binary (stripped) | ≤ 6 MiB | `cargo bloat` + size check | Every PR |

## Techniques

- Normalized haystacks are computed at index time, and the query is normalized once per keystroke.
- The scoring loop reuses buffers and allocates nothing.
- Top-k selection uses `select_nth_unstable_by`, so only the visible results are sorted.
- Catalog and synonyms are `phf` maps and static slices; there is no parsing at startup.
- Only visible rows (at most 12) cross D-Bus, with compact dictionaries; icons travel as names, never as image data.
- The extension pools rows and caches icons, and it creates no actors per keystroke.
- The engine has no timers and no polling: it reacts to file monitors and D-Bus signals.
- Release profile: `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`. Stripping and debug symbols are left to distribution packaging.

