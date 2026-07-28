# Authentication design

This document captures product decisions for multi-user auth in this Q&A service,
plus lessons from reference systems (Stacker News, Routstr, and related token-based AI apps).

It supersedes the earlier “single `ADMIN_PASSWORD` + HMAC” sketch for issue #63.

## Goals

- Support **multiple people**, not one shared admin password.
- Keep **anonymous browse** possible (read-only).
- Support a **lightweight token identity** (minimal profile), separate from full accounts.
- Leave room for **moderator**, **Nostr**, and richer login methods later.
- Enforce auth on the **server** for mutating APIs (UI gating alone is not security).

## Actors and roles (Phase A)

| Actor | How you exist | Capabilities (Phase A) |
|-------|----------------|-------------------------|
| **Anonymous** | No session | Read-only: GET questions/answers |
| **`token` user** | Opaque **sign-in token** only (no username/password/email) | Authenticated; create Q&A; edit/delete **own** content |
| **`user`** | Username + password | Same content powers as `token` for now; richer profile |
| **`admin`** | Bootstrap / promoted full account | Edit/delete **any** content |

### Important distinction: sign-in token ≠ recovery for full accounts

- A **sign-in token user** is its own lightweight account (PPQ / Routstr-style “the secret *is* the identity”).
- It is **not** a NanoGPT-style “backup blob that restores an email/password account.”
- Full accounts (`user` / `admin`) log in with username + password.
- **Anonymous** means *no* credentials and *no* write access—not a token user.

### Later (out of Phase A)

| Concept | Intent |
|---------|--------|
| **Moderator** | Secondary role: moderate others’ content |
| **Nostr** | Login *method* attachable to a `users` row (not a separate role) |
| **Upgrade path** | Link username/password (or Nostr) onto an existing `token` user |
| **Email / OAuth / passkeys** | Optional methods on the same user |

## Target architecture (Phase A)

```text
Anonymous --GET only--> API
token user --Bearer session--> API  (created via /auth/guest-token or login with sign_in_token)
full user/admin --Bearer session--> API  (register / login with password)
```

### Schema (planned)

- `users`
  - `role`: `admin` | `user` | `token`
  - `username` / `password_hash`: required for `user`/`admin`; **NULL** for `token`
  - `display_name`: optional; for `token` e.g. `guest-<shortid>`
- `sessions`: opaque session secret (hash stored), TTL, FK `user_id`
- `sign_in_tokens`: long-lived credential for `token` users (hash stored; plaintext shown once)
- `questions.author_id` / `answers.author_id` → `users(id)`

Bootstrap first admin from `BOOTSTRAP_ADMIN_USERNAME` + `BOOTSTRAP_ADMIN_PASSWORD` when no admin exists. Retire env-password-as-login (`ADMIN_PASSWORD` gate for the whole API).

### API (planned)

| Method | Path | Auth | Behavior |
|--------|------|------|----------|
| POST | `/auth/guest-token` | public | Create `role=token` user + sign-in token; return token once + session |
| POST | `/register` | public | Create `role=user`; return session |
| POST | `/login` | public | `{ username, password }` **or** `{ sign_in_token }` → session |
| POST | `/logout` | session | Revoke session |
| GET | `/me` | session | Current user |
| GET | Q&A | public | Unchanged |
| POST/PUT/DELETE | Q&A | session | Require Bearer; ownership for non-admin |

Rate-limit register/login/guest-token. CORS must allow `Authorization`.

---

## Reference: Stacker News

Repo: https://github.com/stackernews/stacker.news

### What they do

- **One `User` row**, many login methods (not one role per method):
  - Lightning LNURL-auth → `users.pubkey`
  - Nostr → `users.nostrAuthPubkey`
  - Email (hashed), GitHub, Twitter via NextAuth `Account`
  - Optional API key hash on the user
- **Challenge–response** for LN/Nostr:
  1. Server stores short-lived `k1` (`LnAuth`)
  2. Wallet/extension signs challenge
  3. Verify signature; delete `k1` (anti-replay)
  4. Find or create user by pubkey; or **link** pubkey to currently logged-in user
- New crypto users get a **minimal name** (e.g. truncated pubkey)—a real user with little profile.
- Sessions via **NextAuth + httpOnly cookies**, plus multi-account switching (including browse as “anonymous”).
- Strong product advice: **link ≥2 auth methods** so you don’t lock yourself out.

### What to steal

1. User row first; methods second.
2. Minimal identity on first token/crypto signup is OK.
3. Challenge + one-time consume for future Nostr/LN.
4. Link methods later so token users can upgrade.
5. Prefer httpOnly cookies when hardening browser sessions.

### What not to copy in Phase A

NextAuth/Prisma stack, sats economy, full LNURL/NIP-46, multi-account cookie machinery.

### Mapping to us

| Ours | Stacker News analogue |
|------|------------------------|
| Anonymous read-only | No active session / anonymous pointer |
| `token` user | New user from LN/Nostr with tiny profile |
| Full `user` | Email/OAuth-style fuller account |
| Nostr (later) | Method column on same user |
| Admin role | Our addition (SN is less role-driven) |

---

## Reference: Routstr

Site: https://routstr.com  
Core (auth/billing): https://github.com/Routstr/routstr-core  
Docs: https://docs.routstr.com/api/authentication/  
Org: https://github.com/Routstr  

Primary implementation: `routstr/auth.py` (`validate_bearer_key`), plus wallet/balance modules.

### What they do

- **No traditional signup** for API use.
- Identity ≈ **bearer credential**:
  - `Authorization: Bearer sk-…` — stored API key (hash + balance)
  - `Authorization: Bearer cashu…` — eCash token; **SHA-256 hash** becomes key id; redeem into balance
  - Optional `X-Cashu` for per-request pay + change (stateless-ish)
- Keys can be topped up, refunded, rate-limited; **child keys** with limits (delegation).
- **Nostr** is used heavily for **decentralized provider discovery**, not as the main “human login” story for the chat UI.
- Node admin uses a separate `ADMIN_PASSWORD` for operator UI—not the end-user identity model.

### What to steal

1. Opaque bearer as first-class identity (matches our `token` user).
2. Store **hashes** of long-lived secrets, show plaintext once.
3. `Authorization: Bearer …` as the integration shape (OpenAI-compatible habit).
4. Later: optional “child keys” / delegated credentials if we need bot or limited access.
5. Clear error codes for invalid/expired/insufficient (they use 401/402 for paywall).

### What not to copy for a Q&A forum

- Cashu/Lightning **balance** as the core of identity (unless we add paid features).
- Pay-per-request economics.
- Treating Nostr only as discovery (we may want Nostr as **login method** like SN).

### Mapping to us

| Ours | Routstr analogue |
|------|------------------|
| `token` user + sign-in token | `sk-` / hashed Cashu bearer identity |
| Session after login | Reusable `sk-` with server-side record |
| Anonymous read-only | Unauthenticated access to public metadata |
| Full username+password user | Not really Routstr’s consumer model |
| Admin bootstrap | Closer to their node `ADMIN_PASSWORD` (ops), separate from API keys |

---

## Reference: related AI apps (context)

| App | Pattern | Relevance |
|-----|---------|-----------|
| **ppq.ai** | Credit id / API key; no registration required | Same family as Routstr bearer identity |
| **nano-gpt.com** | Multi-method login + “sign-in token” to **restore a session/account** | Different: recovery for an existing account—not our Phase A `token` user |

---

## Phased roadmap

### Phase A (this design)

- Roles: `admin`, `user`, `token`
- Anonymous read-only
- Guest token create + token login
- Username/password register/login
- DB sessions; gate mutating routes; ownership
- Bootstrap admin from env

### Phase B (optional hardening)

- httpOnly session cookies (SN-style) instead of/in addition to Bearer in `localStorage`
- Upgrade `token` → full `user` (set username/password)
- Moderator role + APIs

### Phase C (crypto login)

- Nostr auth (challenge / NIP-07 or NIP-46), link `nostr_pubkey` to `users` (SN-style)
- Optionally LNURL-auth
- Encourage linking a second method

### Explicitly deferred

- Cashu/pay-per-action (Routstr economics)
- OAuth/passkeys/email magic link (unless needed)

## Implementation checklist (engineering)

- [x] Migration: users / sessions / sign_in_tokens / author_id
- [x] argon2 for passwords; hash session + sign-in tokens
- [x] Auth routes + `require_auth` filter on mutating Warp routes
- [x] Frontend: register, password login, continue-with-token / paste token
- [x] Integration tests: 401 without Bearer; token user can post; ownership 403
- [x] Docs: `.env.example`, AGENTS.md, README, `docs/auth-design.md`
- [ ] Close/supersede #63 against this design (via PR)

## References

Index of external apps and repos discussed while designing auth (deep dives above explain how they work):

| Link | Notes |
|------|--------|
| https://ppq.ai/ | Credit id / API key identity; no registration required |
| https://nano-gpt.com/auth/signin | Multi-method login + sign-in token (session/account restore) |
| https://stacker.news/ | Multi-method crypto login (Lightning, Nostr, email, OAuth) |
| https://github.com/stackernews/stacker.news | SN source: challenge–response, linkable auth methods (`lib/auth.js`, NextAuth, `LnAuth`) |
| https://routstr.com/ | Bearer / Cashu token identity; pay-per-request AI |
| https://github.com/Routstr | Routstr organization |
| https://github.com/Routstr/routstr-core | Core auth (`routstr/auth.py`): hashed bearers, `sk-` keys |
| https://docs.routstr.com/api/authentication/ | Routstr authentication API docs |

These informed Phase A (`token` users vs full accounts, anonymous read-only) and later ideas (Nostr as a login method, optional Cashu/pay flows).
