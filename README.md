# ⚡ anchor-cg
┌───────────────────────────────────────────┐
│ │
│ ANCHOR PROGRAM ──► LiteSVM ──► CU DELTA │
│ │
│ measure → save baseline │
│ compare → pass/fail CI │
│ recalibrate → update + audit log │
│ │
└───────────────────────────────────────────┘

text

Deterministic CU baseline tracking for Anchor programs.

---

## What this is

A Rust CLI that measures Compute Unit consumption of Anchor
programs in a deterministic LiteSVM environment. The idea is
to catch real CU regressions in CI while ignoring noise from
framework updates.

## Why I built this

I was debugging CI failures that turned out to be Anchor
framework upgrades inflating CU globally. The tests weren't
wrong — the baseline was. There was no simple tool to handle
this, so I started building one.

## How it works
$ anchor-cg measure --program counter --instruction increment
counter::increment -> 3892 CU
baseline saved to baselines/counter_increment.json

$ anchor-cg compare --program counter
Baseline CU: 3892
Current CU: 3912
Delta: 0.51%
Threshold: 5.00%
OK: within threshold
(exit 0)

$ anchor-cg recalibrate --program counter --reason "anchor 0.31 upgrade"
Recalibrated. CU: 4087 (was: 3892)
Appended to CALIBRATION_LOG.md

text
┌──────────────┬─────────────────────────────────────┐
│ Command │ What it does │
├──────────────┼─────────────────────────────────────┤
│ measure │ Run instruction in LiteSVM, save CU │
│ compare │ Load baseline, check delta, exit 1 │
│ │ if regression exceeds threshold │
│ recalibrate │ Reset baseline, append audit log │
└──────────────┴─────────────────────────────────────┘

text

The measurement is deterministic (LiteSVM, not mainnet), so two
runs on the same commit always produce the same CU. The tool
measures *relative deltas* between commits, not absolute values.

Threshold is configurable (default 5%). If the delta exceeds the
threshold, the CI step fails — otherwise it passes even if the
absolute CU changed due to framework updates.

## Project status

**V1 (current):**
- [x] CLI skeleton with measure, compare, recalibrate
- [x] Baseline save/load with JSON
- [x] Percentage-based threshold comparison
- [x] Append-only calibration log
- [x] Unit tests + scratch tests
- [ ] Real LiteSVM integration (stub CU = 3847 for now)
- [ ] CI workflow YAML

**V2 (planned):**
- Mainnet state forking via RPC for production-accurate profiling
- Multiple instruction baselines per program
- GitHub Action published on marketplace

## Files
anchor-cg/
├── src/
│ ├── main.rs # CLI (clap derive)
│ ├── baseline.rs # Baseline struct, save/load/compare
│ ├── calibrate.rs # recalibrate + audit log
│ └── report.rs # Output formatting (wip)
├── tests/
│ ├── baseline_tests.rs # Unit tests
│ └── scratch.rs # Messy stuff, some #[ignore]
├── baselines/ # Saved CU snapshots (*.json)
├── CALIBRATION_LOG.md # Append-only calibration history
├── notes.md # Developer scratch pad
└── ARCHITECTURE.md # Technical details

text

## Build & Test

```bash
cargo build --release
cargo test
cargo run --release -- measure --program counter --instruction increment
cargo run --release -- compare --program counter
Grant Milestones
text
 M1 ($2,000)     Working CLI + real LiteSVM integration
 ├─ M2 ($1,500)   CI integration + recalibrate improvements
 └─ M3 ($1,500)   GitHub Action + V2 RFC spec
                              Total: $5,000
License
MIT
