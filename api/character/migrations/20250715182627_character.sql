
CREATE TABLE characters (
                       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                       user_id UUID NOT NULL UNIQUE references users(id),
                       name TEXT NOT NULL,
                       level SMALLINT NOT NULL DEFAULT 1,
                       created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE character_movements (
                        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                        character_id UUID NOT NULL UNIQUE references characters(id),
                        pos_x REAL NOT NULL,
                        pos_y REAL NOT NULL,
                        pos_z REAL NOT NULL,
                        dir_x REAL NOT NULL,
                        dir_y REAL NOT NULL,
                        dir_z REAL NOT NULL,
                        mode SMALLINT NOT NULL
);

