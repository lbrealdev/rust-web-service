# justfile for rust

set dotenv-load := true

# Variables

container := 'postgresql'
db_name := 'rustwebdev'

@setup:
    pre-commit install

@run:
    cargo run

@test:
    cargo test

@lint:
    cargo clippy

@fmt:
    cargo fmt

# Postgresql

@create-db:
    docker run -d --name {{ container }} -e POSTGRES_USER=$POSTGRES_USER -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD -p 5432:5432 postgres

@init-db: (create-db)
    echo "Creating database {{ db_name }}..."
    sleep 2
    docker exec {{ container }} psql -U $POSTGRES_USER -c "CREATE DATABASE {{ db_name }};"
