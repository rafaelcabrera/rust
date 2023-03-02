-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR,
    slug VARCHAR NOT NULL,
    body TEXT NOT NULL
)