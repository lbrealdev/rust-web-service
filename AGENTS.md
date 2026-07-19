# AGENTS.md

## Quick start

Database must be running before the server starts. Migrations run automatically on startup.

```
cp .env.example .env   # required: DATABASE_URL + ADMIN_PASSWORD
just db-up             # docker run postgres
just sqlx-migrate      # sqlx migrate run (optional; also runs on boot)
just server            # alias for `just run`; cargo run
```

The server binds to `http://127.0.0.1:3030`.

## Dev commands

- `just server` — run the application (requires DB up + env vars)
- `just test` — `cargo test` (foundation unit tests; no DB required)
- `just test-filter FILTER` — `cargo test FILTER`
- `just lint` — `cargo clippy`
- `just fmt` — `cargo fmt`
- `just watch` — `watchexec` auto-restart on file change

## Testing

- Foundation unit tests cover `extraction_pagination` and `handle-errors` (no Postgres).
- Use `just test` (workspace) or e.g. `just test-filter pagination` / `just test-filter return_error`.
- Store / Warp integration tests against a live DB are not in-tree yet (follow-up).

## Before committing

```
just fmt && just lint && just test
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
- Create question: `static/new-question.html` (loads `new-question.js`; client auth gate + theme persistence)
- Login: inline modal in `index.html`, handled by `main.js`
- CSS: `static/style.css`

## Authentication

- `ADMIN_PASSWORD` is required at process start (no default)
- Login state stored in `localStorage` (`loggedIn` key)
- UI shows create/edit/delete when logged in; create page redirects to `/` if logged out (flash message on home)

### Known gaps

- **No server-side API auth** — mutating endpoints are publicly callable without credentials; UI gating is cosmetic only (tracked as #63)
- Do not treat client-side `localStorage` as security
