-- Your SQL goes here

CREATE TABLE track_tags (
    id SERIAL PRIMARY KEY NOT NULL,
    name text NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)