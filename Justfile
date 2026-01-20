# Justfile = execution contract for agents + humans

set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

default: verify

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
    cargo test --workspace --all-features

build:
    cargo build --workspace --all-features

# Run a specific example
example name:
    cargo run --example {{name}}

# Run a specific binary
bin name *args:
    cargo run --bin {{name}} -- {{args}}

# Run all examples (add yours here)
examples:
    @echo "Define your example smoke tests here"
    # just example my_example

# The standard gate that CI runs
verify: fmt-check clippy test build
    @echo "VERIFY OK"

# Full verification including examples
verify-full: verify examples
    @echo "FULL VERIFY OK"
