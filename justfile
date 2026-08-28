# justfile for rust

set dotenv-load := true

# variables

image := 'postgres'
container := 'postgresql'

# Alias

alias server := run
alias db-up := docker-psql-up
alias db-down := docker-psql-down
alias sqlx-migrate := sqlx-migrate-run
alias sqlx-status := sqlx-migrate-info

# pre-commit

@setup:
    pre-commit install

# cargo

@run:
    cargo run

@check:
    cargo check --workspace

@build:
    cargo build --workspace --release

@clean:
    cargo clean

@update:
    cargo update

@test:
    cargo test --workspace

@test-filter FILTER:
    cargo test --workspace {{FILTER}}

# DB-backed store + Warp integration tests (requires Postgres + DATABASE_URL)
@test-db:
    cargo test --test integration -- --ignored --test-threads=1

@lint:
    cargo clippy --workspace

@fmt:
    cargo fmt

# watchexec

@watch:
    watchexec -e rs,js,css,html just server

# docker

@docker-psql-up:
    echo "Creating container {{ container }}..."
    docker run -d --name {{ container }} -e POSTGRES_USER=$POSTGRES_USER -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD -p 5432:5432 {{ image }}

@docker-psql-down:
    echo "Removing container {{ container }}..."
    docker stop {{ container }}
    docker rm {{ container }}

# sqlx (uses DATABASE_URL from .env)

@sqlx-migrate-add NAME:
    sqlx migrate add -r {{NAME}}

@sqlx-migrate-run:
    sqlx database create
    sqlx migrate run

@sqlx-db-reset:
    just db-down
    just db-up
    sleep 3
    just sqlx-migrate

@sqlx-migrate-info:
    sqlx migrate info

@sqlx-migrate-revert:
    sqlx migrate revert
