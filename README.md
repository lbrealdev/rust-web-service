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
| `BOOTSTRAP_ADMIN_USERNAME` | First admin username if none exists (default `admin`) |
| `BOOTSTRAP_ADMIN_PASSWORD` | First admin password (or legacy `ADMIN_PASSWORD`) |
| `DB_POOL_MAX` | Max DB pool size (optional, default `5`) |
| `RUST_LOG` | Tracing filter (optional) |
| `POSTGRES_USER` / `POSTGRES_PASSWORD` | Used by `just db-up` / migrate recipes |

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/questions` | List questions (`?limit=&offset=`) |
| GET | `/questions/:id` | Get a single question |
| GET | `/questions/:id/answers` | List answers for a question |
| POST | `/questions` | Create question (`201` + JSON; auth required) |
| PUT | `/questions/:id` | Update question (auth + ownership) |
| DELETE | `/questions/:id` | Delete question (auth + ownership; cascades answers) |
| POST | `/answers` | Add answer (`201` + JSON; auth required) |
| DELETE | `/answers/:id` | Delete answer (auth + ownership) |
| POST | `/register` | Create full `user` account |
| POST | `/login` | Login with username/password or `sign_in_token` |
| POST | `/auth/guest-token` | Create lightweight `token` user + sign-in token |
| POST | `/logout` | Revoke session |
| GET | `/me` | Current user |

Common statuses: creates return `201`, missing resources `404`, validation `400`, unauthenticated `401`, ownership `403`. See [docs/api.md](docs/api.md) and [docs/auth-design.md](docs/auth-design.md).

## Authentication

Mutating APIs require `Authorization: Bearer <session>`. Anonymous visitors can browse. Accounts:

- **`token` user** — continue with guest token / paste sign-in token (minimal identity)
- **`user` / `admin`** — username + password (`admin` bootstrapped from env)

See [docs/auth-design.md](docs/auth-design.md) for roles, Stacker News / Routstr notes, and roadmap (Nostr later).

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
