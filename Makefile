.PHONY: fmt fmt-check lint test doc check

fmt:
	cargo fmt --all

lint:
	cargo clippy --workspace --all-targets -- -D warnings

test:
	cargo test --workspace

doc:
	cargo doc --no-deps --open

fmt-check:
	cargo fmt --all -- --check

check: fmt-check lint test