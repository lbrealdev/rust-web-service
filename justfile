# justfile for rust

set dotenv-load := true

@setup:
    pre-commit install

@run:
    cargo run