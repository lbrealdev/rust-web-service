# justfile for rust

set dotenv-load := true

# variables

image := 'postgres'
container := 'postgresql'
db_name := 'rustwebdev'

db_ip := ```
    psql=$(docker ps -a | grep postgres)
    if [ -z "$psql" ]; then
        echo "Container is not running!"
    else
        docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' postgresql
    fi
 ```

docker_ip := if db_ip == "Container is not running!" { "false" } else { "true" }

# Alias

alias db-up := docker-psql-up
alias db-down := docker-psql-down

# pre-commit

@setup:
    pre-commit install

# cargo

@run:
    cargo run

@test:
    cargo test

@lint:
    cargo clippy

@fmt:
    cargo fmt

# docker

@docker-psql-up:
    echo "Creating container {{ container }}..."
    docker run -d --name {{ container }} -e POSTGRES_USER=$POSTGRES_USER -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD -p 5432:5432 postgres

@docker-psql-down:
    echo "Removing container {{ container }}..."
    docker stop {{ container }}
    docker rm {{ container }}

# sqlx

@sql-info:
    if {{ docker_ip }}; then \
        sqlx migrate info -D postgresql://$POSTGRES_USER:$POSTGRES_PASSWORD@{{ db_ip }}:5432/{{ db_name }}; \
    else \
        echo "IP does not exist!"; \
    fi

@sql-init:
    if {{ docker_ip }}; then \
        sqlx database create -D postgresql://$POSTGRES_USER:$POSTGRES_PASSWORD@{{ db_ip }}:5432/{{ db_name }}; \
    else \
        echo "IP does not exist!"; \
    fi

@sql-run:
    if {{ docker_ip }}; then \
        sqlx migrate run -D postgresql://$POSTGRES_USER:$POSTGRES_PASSWORD@{{ db_ip }}:5432/{{ db_name }}; \
    else \
        echo "IP does not exist!"; \
    fi
