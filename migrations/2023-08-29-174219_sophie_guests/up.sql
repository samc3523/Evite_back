-- Your SQL goes here
CREATE TABLE sophie_guests (
  id SERIAL PRIMARY KEY,
  gname VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  phone VARCHAR NOT NULL,
  msg TEXT NOT NULL,
  coming BOOLEAN NOT NULL DEFAULT 't'
)