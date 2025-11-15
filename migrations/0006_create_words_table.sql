-- Migration number: 0006 	 2025-11-15T08:08:43.702Z
PRAGMA defer_foreign_keys = on;
CREATE TABLE IF NOT EXISTS Words (id INTEGER PRIMARY KEY, word TEXT NOT NULL, language_id INTEGER NOT NULL, FOREIGN KEY(language_id) REFERENCES Languages(id));
PRAGMA defer_foreign_keys = off;
