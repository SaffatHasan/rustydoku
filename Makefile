lint:
	cargo fmt
	cargo check
	cargo clippy --fix --allow-dirty

test:
	cargo test
