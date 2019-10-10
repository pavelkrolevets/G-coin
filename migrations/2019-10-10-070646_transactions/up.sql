CREATE TABLE user_transactions (
  id SERIAL PRIMARY KEY,
  usdt_wallet TEXT NOT NULL,
  usdg_wallet TEXT NOT NULL,
  price TEXT NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
)