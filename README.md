# Rust web service

A Q&A web service built with Rust (Warp + Tokio + SQLx + PostgreSQL).

## Quick start

Copy [`.env.example`](.env.example) to `.env` and adjust values as needed:

```sh
cp .env.example .env
just db-up          # start PostgreSQL container
just sqlx-migrate   # run migrations
just server         # start the server on http://127.0.0.1:3030
```

Required env vars (see `.env.example`):

| Variable | Description |
|----------|-------------|
| `DATABASE_URL` | PostgreSQL connection string |
| `ADMIN_PASSWORD` | Password for `POST /login` (required; no default) |
| `DB_POOL_MAX` | Max DB pool size (optional, default `5`) |
| `RUST_LOG` | Tracing filter (optional) |
| `POSTGRES_USER` / `POSTGRES_PASSWORD` | Used by `just db-up` / migrate recipes |

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/questions` | List questions (`?limit=&offset=`) |
| GET | `/questions/:id` | Get a single question |
| GET | `/questions/:id/answers` | List answers for a question |
| POST | `/questions` | Create question (`201` + JSON body) |
| PUT | `/questions/:id` | Update question |
| DELETE | `/questions/:id` | Delete question (cascades answers) |
| POST | `/answers` | Add answer (`201` + JSON body) |
| DELETE | `/answers/:id` | Delete answer |
| POST | `/login` | Admin login |

Common statuses: creates return `201`, missing resources return `404`, empty title/content return `400`. See [docs/api.md](docs/api.md) for full request/response schemas.

## Authentication

The UI includes a simple login gate. Once logged in, create/edit/delete controls become visible (including edit on the question detail page). Create-question (`/new-question.html`) redirects home if you are not logged in.

This gating is **client-side only**. The API has **no auth middleware** — anyone can call mutating endpoints directly. Server-side auth is a known gap (see issue #63).

The password is read from `ADMIN_PASSWORD` in `.env` (required at startup; copy from `.env.example`).

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

| Layer | Technology |
|-------|------------|
| HTTP framework | Warp |
| Async runtime | Tokio |
| Database | PostgreSQL (via Docker) |
| ORM/Migrations | SQLx |
| Error handling | Local crate `handle-errors/` |
| Frontend | Vanilla HTML/CSS/JS in `static/` |

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
cp .env.example .env
just db-up
just sqlx-migrate
```
