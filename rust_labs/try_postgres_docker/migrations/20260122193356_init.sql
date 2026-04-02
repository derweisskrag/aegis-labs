-- Add migration script here
CREATE TABLE IF NOT EXISTS pythons (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);