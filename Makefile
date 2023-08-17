lint:
	cargo fmt
	cargo check
	cargo clippy --fix --allow-dirty --allow-staged

test:
	cargo test

install:
	@echo "Installing pre-commit hook..."
	@echo "#!/bin/sh" > .git/hooks/pre-commit
	@echo "make lint test" >> .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-commit
	@echo "Pre-commit hook installed."
