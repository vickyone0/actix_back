-- Your SQL goes here
CREATE TABLE products (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  price DECIMAL NOT NULL
);