
CREATE TABLE characters (
                       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                       user_id UUID NOT NULL UNIQUE references users(id),
                       name TEXT NOT NULL,
                       level SMALLINT NOT NULL DEFAULT 1,
                       created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
-- Add migration script here
