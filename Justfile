default: choose

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
