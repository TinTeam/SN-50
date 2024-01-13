.PHONY: all install pre-commit check check-fmt check-clippy fix fix-fmt fix-clippy test test-cov build

all: install fix test build

install:
	cargo install git-cliff --locked
	cargo install cargo-tarpaulin --locked
	pre-commit install

pre-commit:
	pre-commit run --all --verbose

check: check-fmt check-clippy

check-fmt:
	cargo fmt --all --check

check-clippy:
	cargo clippy --all-targets --all-features --locked -- -D warnings

fix: fix-fmt fix-clippy

fix-fmt:
	cargo fmt --all

fix-clippy:
	cargo clippy --all-targets --all-features --fix --locked

test:
	cargo test --all-targets --locked

test-cov:
	cargo tarpaulin --all-targets --locked

build:
	cargo build --all-targets --locked
