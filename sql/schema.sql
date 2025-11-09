DROP TABLE IF EXISTS Languages;
CREATE TABLE IF NOT EXISTS Languages (id INTEGER PRIMARY KEY, name TEXT NOT NULL, name_other TEXT, code TEXT NOT NULL);
INSERT INTO Languages (id, name, name_other, code) VALUES (0, "Deutsch", "German", "de"), (1, "English", "English", "en");
