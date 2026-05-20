CREATE TABLE IF NOT EXISTS users (
                                     id UUID PRIMARY KEY,
                                     username TEXT NOT NULL UNIQUE,
                                     email TEXT NOT NULL UNIQUE,
                                     password_hash TEXT NOT NULL,
                                     is_active BOOLEAN NOT NULL DEFAULT TRUE,
                                     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    );

CREATE TABLE IF NOT EXISTS wallets (
                                       id UUID PRIMARY KEY,
                                       user_id UUID NOT NULL UNIQUE REFERENCES users(id),
    balance REAL NOT NULL DEFAULT 0.0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    );

CREATE TABLE IF NOT EXISTS matches (
                                       id UUID PRIMARY KEY,
                                       sport TEXT NOT NULL,
                                       league TEXT NOT NULL,
                                       home_team TEXT NOT NULL,
                                       away_team TEXT NOT NULL,
                                       starts_at TIMESTAMP NOT NULL,
                                       status TEXT NOT NULL DEFAULT 'Scheduled',
                                       home_score INTEGER,
                                       away_score INTEGER,
                                       result TEXT,
                                       created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    );

CREATE TABLE IF NOT EXISTS markets (
                                       id UUID PRIMARY KEY,
                                       match_id UUID NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    is_open INTEGER NOT NULL DEFAULT 1,
    is_settled INTEGER NOT NULL DEFAULT 0
    );

CREATE TABLE IF NOT EXISTS selections (
                                          id UUID PRIMARY KEY,
                                          market_id UUID NOT NULL REFERENCES markets(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    odds REAL NOT NULL,
    result TEXT NOT NULL DEFAULT 'Pending',
    is_active INTEGER NOT NULL DEFAULT 1
    );

CREATE TABLE IF NOT EXISTS bets (
                                    id UUID PRIMARY KEY,
                                    user_id UUID NOT NULL REFERENCES users(id),
    market_id UUID NOT NULL REFERENCES markets(id),
    selection_id UUID NOT NULL REFERENCES selections(id),
    stake REAL NOT NULL,
    odds REAL NOT NULL,
    potential_win REAL NOT NULL,
    actual_win REAL,
    status TEXT NOT NULL DEFAULT 'Pending',
    placed_at TIMESTAMP NOT NULL DEFAULT NOW(),
    settled_at TEXT
    );

CREATE TABLE IF NOT EXISTS transactions (
                                            id UUID PRIMARY KEY,
                                            wallet_id UUID NOT NULL REFERENCES wallets(id),
    type TEXT NOT NULL,
    amount REAL NOT NULL,
    balance_before REAL NOT NULL,
    balance_after REAL NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    bet_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);