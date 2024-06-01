CREATE TABLE lists (
    uid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT DEFAULT '' NOT NULL,
    author_uid UUID REFERENCES persons (uid) ON DELETE CASCADE NOT NULL,
    tasks UUID[] DEFAULT '{}' NOT NULL
);
