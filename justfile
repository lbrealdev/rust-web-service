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

alias server := run
alias db-up := docker-psql-up
alias db-down := docker-psql-down
#alias sqlx-create := sqlx-database-create
alias sqlx-migrate := sqlx-migrate-run
alias sqlx-status := sqlx-migrate-info

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

# sqlx

# @sqlx-database-create:
#     if {{ docker_ip }}; then \
#         sqlx database create -D postgresql://$POSTGRES_USER:$POSTGRES_PASSWORD@{{ db_ip }}:5432/{{ db_name }}; \
#     else \
#         echo "IP does not exist!"; \
#     fi

@_sqlx-database-create:
    echo "Creating {{ db_name }} database"
    sqlx database create -D postgresql://$POSTGRES_USER:$POSTGRES_PASSWORD@{{ db_ip }}:5432/{{ db_name }}

@_sqlx-migrate-add:
    sqlx migrate add -r questions_table
    sleep 1
    sqlx migrate add -r answers_table

@sqlx-migrate-run:
    echo "Running migrations..."
    sleep 3
    # just _sqlx-database-create
    # just _sqlx-migrate-add
    sqlx migrate run -D postgresql://$POSTGRES_USER:$POSTGRES_PASSWORD@{{ db_ip }}:5432/{{ db_name }}

@sqlx-run:
    echo '{{ if db_ip == "Container is not running!" { "Error!" } else { `just sqlx-migrate` } }}'

@sqlx-migrate-info:
    if {{ docker_ip }}; then \
        sqlx migrate info -D postgresql://$POSTGRES_USER:$POSTGRES_PASSWORD@{{ db_ip }}:5432/{{ db_name }}; \
    else \
        echo "IP does not exist!"; \
    fi

@sqlx-migrate-revert:
    sqlx migrate revert -D postgresql://$POSTGRES_USER:$POSTGRES_PASSWORD@{{ db_ip }}:5432/{{ db_name }}