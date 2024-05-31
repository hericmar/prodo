CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE persons (
    uid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name VARCHAR(32) NOT NULL,
    surname VARCHAR(32) NOT NULL,
    username VARCHAR(32) UNIQUE NOT NULL,
    email VARCHAR(320) UNIQUE NOT NULL,
    password TEXT NOT NULL
)
