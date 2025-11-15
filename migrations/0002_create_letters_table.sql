-- Migration number: 0002 	 2025-11-10T20:03:24.955Z
PRAGMA defer_foreign_keys = on;
CREATE TABLE IF NOT EXISTS Letters (id INTEGER PRIMARY KEY, letter TEXT NOT NULL, language_id INTEGER NOT NULL, regular BOOLEAN, hidden BOOLEAN, name_en TEXT, FOREIGN KEY(language_id) REFERENCES Languages(id));
PRAGMA defer_foreign_keys = off;
