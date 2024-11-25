-- Create basic users table for fun stuff around the bot
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  discord_id VARCHAR(255) NOT NULL UNIQUE,
  actions_allowed TINYINT DEFAULT 1,
  about TEXT,
  pronouns VARCHAR(255)
);

