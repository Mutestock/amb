-- Your SQL goes here

CREATE TABLE images (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users(id),
    title TEXT NOT NULL,
    path TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)
