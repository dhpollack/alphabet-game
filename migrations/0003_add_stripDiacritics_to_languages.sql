-- Migration number: 0003 	 2025-11-12T18:05:39.992Z
ALTER TABLE Languages ADD COLUMN stripDiacritics BOOLEAN DEFAULT true NOT NULL;
