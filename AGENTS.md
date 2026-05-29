# AGENTS.md

## Quick start

Database must be running and migrations applied before the server starts.

```
just db-up          # docker run postgres
just sqlx-migrate   # sqlx migrate run
just server         # alias for `just run`; cargo run
```

The server binds to `http://127.0.0.1:3030`.

## Dev commands

- `just server` — run the application (requires DB up + migrated)
- `just test` — `cargo test`
- `just lint` — `cargo clippy`
- `just fmt` — `cargo fmt`
- `just watch` — `watchexec` auto-restart on file change

## Before committing

```
just fmt && just lint
```

## Database

- PostgreSQL container started via `just db-up` (container name: `postgresql`)
- Connection string hardcoded in `src/main.rs`: `postgres://admin:localpsql2025@localhost:5432/rustwebdev`
- Credentials in `.env`: `POSTGRES_USER=admin`, `POSTGRES_PASSWORD=localpsql2025`
- Migrations embedded via `sqlx::migrate!()` and run automatically on startup in `main.rs`
- Migration files live in `migrations/`

## Architecture

- Warp web framework with Tokio runtime.
- Routes: `src/routes/`
- Types: `src/types/`
- Store (DB queries): `src/store.rs`
- Custom error library: local crate `handle-errors/`
- Static assets served from `static/` by `warp::fs::dir("static")`

## Static files

- Entry page: `static/index.html` (loads `main.js`)
- Create question: `static/new-question.html` (loads `new-question.js`)
- Login: inline modal in `index.html`, handled by `main.js`
- CSS: `static/style.css`

## Authentication

- Simple password gate
- Login state stored in `localStorage` (`loggedIn` key)
- API endpoints have no auth — gating is client-side only
