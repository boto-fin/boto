set dotenv-load := true

default: ci

fmt:
    cargo fmt --all -- --check

check:
    cargo check --workspace --all-targets --locked

test *args:
    ./tools/test {{args}}

clippy:
    cargo clippy --workspace --all-targets --locked -- --deny warnings

crap *args:
    cargo llvm-cov --workspace --all-targets --locked --lcov --output-path lcov.info
    cargo crap --lcov lcov.info {{args}}

crap-check:
    just crap --fail-above --threshold 30

ci: fmt check clippy test

run *args:
    cargo run {{args}}

nix-check:
    nix flake check

nix-build:
    nix build

nix-develop:
    nix develop
