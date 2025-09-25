-- Add up migration script here
CREATE TABLE IF NOT EXISTS players (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    is_owner BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
-- Enforce exactly one owner (partial unique index)
CREATE UNIQUE INDEX uniq_single_owner ON players ((is_owner))
WHERE is_owner = true;