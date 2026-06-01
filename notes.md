# dev notes

## june 2
- basic CLI skeleton finally compiles, took way too long
- stub CU is 3847, pulled that number out of thin air
  need to replace with actual litesvm measurement soon
- compare command does exit code 1 on regression now, tested
- recalibrate appends to CALIBRATION_LOG.md, seems fine
  but the append isnt atomic, should fix if we get concurrent runs
- TODO:
  - [x] clap subcommands working
  - [x] baseline save/load roundtrip
  - [x] compare logic with threshold
  - [ ] real litesvm integration (big one)
  - [ ] figure out what instruction to use as default test
  - [ ] CI workflow yaml

## june 1
- project kicked off, sketched the architecture
- decided on 5% default threshold, totally arbitrary
  might be too tight for programs that use PDAs heavily
- anchor 0.30 -> 0.31 bumped CU globally, that was the trigger
  for this whole project. wasted a whole day debugging a
  false positive in CI. never again.
- recalibration log format is timestamp | reason | old -> new
  simple but works for now
- need to check: does litesvm handle sysvars correctly?
  not sure if rent and clock are available in the minimal env

## may 28
- tried using BankClient directly from solana-program-test
  but the API is really awkward, litesvm is much cleaner
- sketched out the V2 idea: mainnet RPC forking for
  production-accurate profiling. need to write the RFC doc
  for the grant application
