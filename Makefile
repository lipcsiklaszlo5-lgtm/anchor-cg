.PHONY: build test demo clean

build:
	cargo build --release

test:
	cargo test

demo: build
	@echo "╔══════════════════════════════════════╗"
	@echo "║   anchor-cg DEMO                     ║"
	@echo "╚══════════════════════════════════════╝"
	@echo ""
	@echo "▶ measure:"
	@cargo run --release -- measure --program counter --instruction increment
	@echo ""
	@echo "▶ recalibrate:"
	@cargo run --release -- recalibrate --program counter --reason "demo init"
	@echo ""
	@echo "▶ compare (should PASS):"
	@cargo run --release -- compare --program counter; echo "Exit: $$?"
	@echo ""
	@echo "▶ calibration log:"
	@cat CALIBRATION_LOG.md

clean:
	cargo clean
	rm -rf baselines/*.json
	rm -f CALIBRATION_LOG.md
