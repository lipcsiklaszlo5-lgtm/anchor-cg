#!/bin/bash
set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}╔══════════════════════════════════════╗"
echo -e "║   ⚡ anchor-cg DEMO ⚡                ║"
echo -e "╚══════════════════════════════════════╝${NC}"
echo ""

cargo build --release 2>/dev/null

echo -e "${YELLOW}▶ measure --program counter --instruction increment${NC}"
cargo run --release -- measure --program counter --instruction increment
echo ""

echo -e "${YELLOW}▶ recalibrate --program counter --reason 'demo baseline'${NC}"
cargo run --release -- recalibrate --program counter --reason "demo baseline"
echo ""

echo -e "${YELLOW}▶ compare --program counter${NC}"
if cargo run --release -- compare --program counter; then
    echo -e "${GREEN}  ▲ CI PASSED (within threshold)${NC}"
else
    echo -e "${RED}  ▼ CI FAILED (regression detected)${NC}"
fi
echo ""

echo -e "${BLUE}▶ CALIBRATION_LOG.md:${NC}"
cat CALIBRATION_LOG.md
