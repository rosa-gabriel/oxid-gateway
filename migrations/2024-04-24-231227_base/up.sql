-- Your SQL goes here
CREATE TABLE target (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  host VARCHAR NOT NULL,
  port INTEGER NOT NULL
)
