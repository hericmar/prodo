CREATE TABLE calendar_subscriptions (
    secret VARCHAR(255) PRIMARY KEY,
    person_uid UUID UNIQUE NOT NULL REFERENCES persons(uid) ON DELETE CASCADE
)
