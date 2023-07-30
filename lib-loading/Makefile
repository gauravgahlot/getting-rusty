.PHONY: build
build:
	@echo "Cleaning..."
	cargo clean

	@echo "Building..."
	cargo build --release

.PHONY: run
run: build
	@echo "Running..."
	cargo run -p executor --release

