CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    role VARCHAR(16) NOT NULL CHECK (role IN ('admin', 'user', 'token')),
    username VARCHAR(64) UNIQUE,
    password_hash TEXT,
    display_name VARCHAR(128),
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT users_full_account_creds CHECK (
        (role = 'token' AND username IS NULL AND password_hash IS NULL)
        OR (role IN ('admin', 'user') AND username IS NOT NULL AND password_hash IS NOT NULL)
    )
);

CREATE TABLE IF NOT EXISTS sessions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash CHAR(64) NOT NULL UNIQUE,
    expires_at TIMESTAMP NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS sessions_user_id_idx ON sessions(user_id);
CREATE INDEX IF NOT EXISTS sessions_expires_at_idx ON sessions(expires_at);

CREATE TABLE IF NOT EXISTS sign_in_tokens (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    token_hash CHAR(64) NOT NULL UNIQUE,
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);

ALTER TABLE questions
    ADD COLUMN IF NOT EXISTS author_id INTEGER REFERENCES users(id) ON DELETE SET NULL;

ALTER TABLE answers
    ADD COLUMN IF NOT EXISTS author_id INTEGER REFERENCES users(id) ON DELETE SET NULL;
