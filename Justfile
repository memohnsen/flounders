default: choose

choose:
    @just choose

test:
    codecrafters test

submit:
    codecrafters submit

lint:
    cargo clippy

build:
    cargo build

run:
    cargo run
