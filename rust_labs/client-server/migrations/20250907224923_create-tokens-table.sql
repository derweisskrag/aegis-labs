-- Add migration script here
CREATE TABLE tokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    refresh_token TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id)
);


