-- Add up migration script here
CREATE TABLE IF NOT EXISTS assets (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  name TEXT NOT NULL UNIQUE,
  unit_value DOUBLE PRECISION NOT NULL
);

INSERT INTO assets (name, unit_value) VALUES
  ('Bitcoin', 30.0),
  ('Dolar', 5.15),
  ('Ethereum', 20.0),
  ('Euro', 5.93),
  ('Cardano', 1.5),
  ('Solana', 30.0),
  ('Polkadot', 20.0);
