# V2 Roadmap: Mainnet State Forking

## Goal

Production-accurate CU profiling by forking mainnet state into
a local LiteSVM instance, then executing the program against
real account data instead of synthetic test fixtures.

## Motivation

V1 answers: "did this commit increase CU usage?"
V2 will answer: "will this transaction succeed on mainnet?"

The difference: V1 measures relative deltas, V2 measures
absolute limits against real validator conditions.

## Technical Approach

### Step 1: RPC State Download
anchor-cg fork --rpc https://api.mainnet-beta.solana.com
--program counter
--slot 123456789

text

This downloads all accounts owned by the program, plus any
accounts referenced in upcoming transactions (PDAs, token
accounts, sysvars). Accounts are serialized into a local
snapshot directory.

### Step 2: LiteSVM Fork Creation
let snapshot = Snapshot::load("snapshots/counter_slot_123456789");
let svm = LiteSVM::from_snapshot(snapshot);
let result = svm.send_transaction(tx);

text

LiteSVM is initialized with the downloaded account state,
replacing the default genesis configuration.

### Step 3: Deterministic Replay

Once the snapshot exists, subsequent runs are fully local
and deterministic. No RPC needed. CI caches the snapshot.

## Challenges

### Download Size
Some programs have thousands of accounts. A token program
might need all mint accounts. Mitigation:
- Filter by program owner
- Only download accounts needed for the specific instruction
- Cache snapshots in CI (GitHub Actions cache, S3, etc)

### RPC Reliability
Public RPC endpoints have rate limits and can be slow.
Mitigation:
- Support multiple RPC providers (Helius, Triton, QuickNode)
- Configurable timeouts and retry logic
- Fail gracefully: if RPC is down, fall back to V1 delta mode

### State Freshness
Mainnet state changes every slot (~400ms). A snapshot from
slot N might be stale by slot N+100. Mitigation:
- Lock to a specific slot for the entire CI run
- Store slot number in the snapshot manifest
- Allow manual slot pinning for release branches

### Cost
Downloading state consumes RPC credits on paid providers.
Mitigation:
- First-party support for public RPC (free, slow)
- Snapshot caching so cost is amortized
- Estimate: typical program needs 50-500 accounts,
  ~10-100KB download per CI run

## API Sketch
anchor-cg fork
--program <NAME>
--rpc <URL>
--slot <NUMBER>
--output snapshots/

anchor-cg measure
--program <NAME>
--instruction <NAME>
--snapshot snapshots/counter_slot_123456789

anchor-cg compare
--program <NAME>
--snapshot snapshots/counter_slot_123456789

text

The `--snapshot` flag is optional. Without it, V1 behavior
(synthetic LiteSVM). With it, V2 behavior (forked mainnet).

## Timeline
Week 1-2: Research RPC account download formats
Week 3-4: Prototype snapshot serialization
Week 5-6: Integrate with LiteSVM fork API
Week 7-8: CI caching strategy + docs
Week 9: Public RFC + community feedback

text
