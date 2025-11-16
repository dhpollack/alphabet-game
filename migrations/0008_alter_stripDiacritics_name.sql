-- Migration number: 0008 	 2025-11-16T19:03:00.729Z
ALTER TABLE Languages RENAME COLUMN stripDiacritics TO strip_diacritics;
