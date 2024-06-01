CREATE TABLE tasks (
    uid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    summary VARCHAR(255) DEFAULT '' NOT NULL,
    description TEXT DEFAULT '' NOT NULL,
    completed TIMESTAMPTZ DEFAULT NULL,
    created TIMESTAMPTZ DEFAULT now() NOT NULL,
    author_uid UUID REFERENCES persons (uid) ON DELETE CASCADE NOT NULL
)