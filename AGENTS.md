# AGENTS.md

## Quick start

Database must be running before the server starts. Migrations run automatically on startup.

```
cp .env.example .env   # required: DATABASE_URL + BOOTSTRAP_ADMIN_PASSWORD (or ADMIN_PASSWORD)
just db-up             # docker run postgres
just sqlx-migrate      # sqlx migrate run (optional; also runs on boot)
just server            # alias for `just run`; cargo run
```

The server binds to `http://127.0.0.1:3030`.

## Dev commands

- `just server` — run the application (requires DB up + env vars)
- `just test` — `cargo test` (unit tests; no DB required)
- `just test-filter FILTER` — `cargo test FILTER`
- `just test-db` — ignored DB integration tests (requires Postgres + `DATABASE_URL`)
- `just lint` — `cargo clippy`
- `just fmt` — `cargo fmt`
- `just watch` — `watchexec` auto-restart on file change

## Testing

- Unit tests cover `extraction_pagination` and `handle-errors` (no Postgres).
- Use `just test` (workspace) or e.g. `just test-filter pagination` / `just test-filter handle`.
- DB-backed store + Warp API tests live in `tests/integration.rs` and are `#[ignore]`d by default.
- Run them with Postgres up and `.env` loaded: `just db-up && just sqlx-migrate && just test-db`
  (or any reachable `DATABASE_URL`; tests migrate on connect and clean up created rows).
- API filter factory: `routes::api(store)` in `src/routes/api.rs` (used by `main` and Warp tests).

## Before committing

```
just fmt && just lint && just test
```

## Database

- PostgreSQL container started via `just db-up` (container name: `postgresql`)
- Connection string from env: `DATABASE_URL` (see `.env.example`)
- Credentials in `.env`: `POSTGRES_USER`, `POSTGRES_PASSWORD`, `DATABASE_URL`, `BOOTSTRAP_ADMIN_USERNAME` / `BOOTSTRAP_ADMIN_PASSWORD` (or legacy `ADMIN_PASSWORD`)
- Optional: `DB_POOL_MAX` (default `5`), `RUST_LOG`
- Auth design notes: [`docs/auth-design.md`](docs/auth-design.md)
- Migrations embedded via `sqlx::migrate!()` and run automatically on startup in `main.rs`
- Migration files live in `migrations/`

## Architecture

- Warp web framework with Tokio runtime.
- Library crate: `src/lib.rs` (`routes`, `store`, `types`); binary `src/main.rs` wires env/DB/static/CORS
- Routes: `src/routes/` (`routes::api(store)` builds the API filter)
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

- Multi-user auth (Phase A): anonymous read-only; `token` users (sign-in token); `user` / `admin` (username + password)
- Bootstrap first admin from `BOOTSTRAP_ADMIN_*` (or `ADMIN_PASSWORD`) when none exists
- Mutating APIs require `Authorization: Bearer <session>`; ownership enforced (admin can modify any)
- Frontend stores `sessionToken` + `authUser` in `localStorage` (see `static/auth.js`)
- Design notes and Stacker News / Routstr references: [`docs/auth-design.md`](docs/auth-design.md)

### Later

- Moderator role, Nostr login method, httpOnly cookies (see auth-design roadmap)
