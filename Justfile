default: choose

# Enter the Nix development shell without direnv.
nix:
    nix develop

choose:
    @just choose

test:
    codecrafters test

run:
    codecrafters submit

lint:
    cargo clippy

build:
    cargo build
