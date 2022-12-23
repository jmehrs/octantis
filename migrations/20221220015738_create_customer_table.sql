-- Add migration script here
-- Create Customer Table
CREATE TABLE customer(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL,
    created_at timestamptz NOT NULL
);