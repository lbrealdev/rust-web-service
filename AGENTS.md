# AGENTS.md

## Quick start

Database must be running before the server starts. Migrations run automatically on startup.

```
cp .env.example .env   # if needed
just db-up             # docker run postgres
just sqlx-migrate      # sqlx migrate run (optional; also runs on boot)
just server            # alias for `just run`; cargo run
```

The server binds to `http://127.0.0.1:3030`.

## Dev commands

- `just server` — run the application (requires DB up + env vars)
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
- Connection string from env: `DATABASE_URL` (see `.env.example`)
- Credentials in `.env`: `POSTGRES_USER`, `POSTGRES_PASSWORD`, `DATABASE_URL`, `ADMIN_PASSWORD`
- Optional: `DB_POOL_MAX` (default `5`), `RUST_LOG`
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
- Question detail: `static/question.html` (loads `question.js`) — view, edit (logged-in), answers
- Create question: `static/new-question.html` (loads `new-question.js`)
- Login: inline modal in `index.html`, handled by `main.js`
- CSS: `static/style.css`

## Authentication

- Simple password gate (`ADMIN_PASSWORD` required at process start)
- Login state stored in `localStorage` (`loggedIn` key)
- **Known gap:** API endpoints have no server-side auth — gating is client-side only
