-- Add migration script here
CREATE TABLE IF NOT EXISTS todo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    message TEXT NOT NULL,
    is_done BOOLEAN NOT NULL CHECK (is_done IN (0, 1)),
    created_at INTEGER NOT NULL
);
