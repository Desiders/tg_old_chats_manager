run *argument:
    cargo run -- {{argument}}

run-release *argument:
    cargo run --release -- {{argument}}

clippy:
    cargo clippy --all --all-features -- -W clippy::pedantic

lint: clippy

format:
    cargo fmt --all

fmt: format
