CREATE TABLE IF NOT EXISTS `users` (
    email               TEXT PRIMARY KEY,
    key_contents        TEXT NOT NULL,
    -- The hash of the code for confirming that the email is valid
    confirmation_code   TEXT NOT NULL,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS `idx_unique_email` ON `users` (email);
