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

ci: fmt check clippy test

run *args:
    cargo run {{args}}

nix-check:
    nix flake check

nix-build:
    nix build

nix-develop:
    nix develop
