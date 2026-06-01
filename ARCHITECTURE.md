# Architecture: anchor-cg V1

## Overview

anchor-cg is a Rust CLI that provides deterministic CU baseline
tracking for Solana Anchor programs. It uses LiteSVM as the
execution environment and focuses on *relative delta detection*
rather than absolute CU accuracy.

## Design Decisions

### Why LiteSVM?

| Option | Pros | Cons |
|--------|------|------|
| solana-program-test | Built-in, well-known | Heavy, slow, BankClient API is awkward |
| LiteSVM | Lightweight, fast, clean API | Younger project, less ecosystem support |
| Mainnet RPC | Production-accurate | Non-deterministic, requires network |

LiteSVM was chosen because V1 needs deterministic, CI-friendly
execution. Mainnet forking is planned for V2 when absolute
accuracy matters.

### Relative Deltas vs Absolute CU
Problem:
Anchor 0.31 inflates all CU by ~5%
→ CI fails on every program
→ Developers ignore CI (alert fatigue)

Solution:
measure baseline → compare HEAD vs baseline
→ if delta < threshold: pass (even if absolute CU changed)
→ if delta > threshold: fail (real regression)

text

This means the tool doesn't care about "what is the exact CU on
mainnet" — it cares about "did this commit make it worse".

### Threshold Logic
delta_pct = ((current_cu - baseline_cu) / baseline_cu) * 100

if delta_pct > threshold_pct:
exit 1 (CI fail)
else:
exit 0 (CI pass)

text

Default threshold is 5%. This is configurable per baseline.

Why percentage and not absolute?
- Different programs have different CU profiles
- A +50 CU increase on a 500 CU program is 10% (significant)
- A +50 CU increase on a 5000 CU program is 1% (noise)
- Percentage scales correctly

## Data Flow
┌──────────┐ ┌──────────────┐ ┌─────────────────┐
│ User │ │ anchor-cg │ │ Filesystem │
├──────────┤ ├──────────────┤ ├─────────────────┤
│ │ │ │ │ │
│ measure ─┼────►│ run LiteSVM ├────►│ baselines/ │
│ │ │ get CU │ │ prog_inst.json│
│ │ │ │ │ │
│ compare ─┼────►│ load baseline├────►│ stdout │
│ │ │ run LiteSVM │ │ exit 0 or 1 │
│ │ │ compare │ │ │
│ │ │ │ │ │
│ recalib.─┼────►│ get new CU ├────►│ baselines/ │
│ │ │ save baseline│ │ CALIBRATION_LOG │
│ │ │ append log │ │ │
└──────────┘ └──────────────┘ └─────────────────┘

text

## Baseline JSON Format

```json
{
  "program": "counter",
  "instruction": "increment",
  "commit": "a3f9c2d",
  "cu_consumed": 3892,
  "threshold_pct": 5.0,
  "timestamp": "2025-06-01T10:23:11Z",
  "anchor_version": "0.31.0"
}
Fields:

program / instruction: which code path was measured

commit: git SHA at time of measurement

cu_consumed: compute units used (from LiteSVM transaction metadata)

threshold_pct: max allowed increase before CI fails

timestamp: ISO 8601, when baseline was created

anchor_version: Anchor version used (for debugging inflation)

Calibration Log
CALIBRATION_LOG.md is append-only. Format:

text
TIMESTAMP | REASON | old: X CU -> new: Y CU
Design properties:

Append-only (never overwrites history)

Human-readable (markdown, can be viewed in GitHub)

Machine-parseable (pipe-delimited, consistent format)

Write-once semantics enforced by convention (the CLI only appends)

V2: Mainnet State Forking (RFC Preview)
V2 will add anchor-cg measure --fork mainnet using RPC to
download account state before executing. This gives production-
accurate CU profiling while maintaining deterministic replay.

text
┌──────────┐    ┌──────────────┐    ┌─────────────┐
│ Mainnet  │    │ anchor-cg V2 │    │ LiteSVM     │
│ RPC      ├───►│              ├───►│ (local fork) │
│          │    │ fetch state   │    │ execute tx  │
└──────────┘    └──────────────┘    └──────┬───────┘
                                           │
                                           ▼
                                     CU measurement
Challenges to solve in V2:

Large account downloads (need caching)

RPC latency in CI (need timeout/retry)

State freshness guarantees (use specific slot)

Cost (RPC providers charge per request)

Error Handling
The CLI uses anyhow throughout. All functions return Result<T>.
No panic!() outside of tests. Exit codes:

text
0  →  success (compare passed, or measure/calibrate completed)
1  →  regression detected (compare failed) or runtime error
Testing Strategy
text
tests/baseline_tests.rs  →  Proper unit tests, clean
tests/scratch.rs         →  Work-in-progress, some #[ignore]
Tests cover:

Baseline JSON roundtrip (save → load → compare)

Threshold logic (should fail when delta > threshold)

Stub value consistency (will be replaced with real LiteSVM)
