# Enter the Nix development shell without direnv.
nix:
    nix develop

test:
    codecrafters test

run:
    codecrafters submit

run-local:
    cargo build
    ./target/debug/codecrafters-interpreter tokenize ./test.lox

build:
    cargo build

test-local:
    cargo check
    cargo clippy --all-targets --locked -- -D warnings
    cargo test
