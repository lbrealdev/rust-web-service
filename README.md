# Rust web service

A Q&A web service built with Rust (Warp + Tokio + SQLx + PostgreSQL).

## Quick start

```sh
just db-up          # start PostgreSQL container
just sqlx-migrate   # run migrations
just server         # start the server on http://127.0.0.1:3030
```

## Endpoints

| Method | Path               | Description       |
|--------|--------------------|-------------------|
| GET    | `/questions`       | List questions    |
| POST   | `/questions`       | Create question   |
| PUT    | `/questions/:id`   | Update question   |
| DELETE | `/questions/:id`   | Delete question   |
| POST   | `/answers`         | Add answer        |
| POST   | `/login`           | Admin login       |

See [docs/api.md](docs/api.md) for full request/response schemas.

## Authentication

The UI includes a simple login gate. Once logged in, the **Create New Question** button and **Delete** buttons become visible. This is client-side only — the API itself has no auth headers.

The password is set in `.env` as `ADMIN_PASSWORD`.

## Dev commands

```sh
just lint    # cargo clippy
just fmt     # cargo fmt
just test    # cargo test
just watch   # auto-restart on file change
```

Before committing:

```sh
just fmt && just lint && just test
```

## Architecture

| Layer          | Technology                       |
|----------------|----------------------------------|
| HTTP framework | Warp                             |
| Async runtime  | Tokio                            |
| Database       | PostgreSQL (via Docker)          |
| ORM/Migrations | SQLx                             |
| Error handling | Local crate `handle-errors/`     |
| Frontend       | Vanilla HTML/CSS/JS in `static/` |

## Project structure

```
src/
  main.rs          Entry point, Warp routes
  store.rs         Database queries
  routes/          Route handlers (question, answer, login)
  types/           Request/response structs
handle-errors/     Custom error types
migrations/        SQL migration files
static/            Frontend assets
docs/              API documentation
```

## Setup database

```sh
docker pull postgres
echo 'POSTGRES_PASSWORD="localpsql2025"' > .env
just db-up
```
