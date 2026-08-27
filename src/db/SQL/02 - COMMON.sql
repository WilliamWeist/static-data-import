BEGIN;

DROP TABLE IF EXISTS name;
DROP TABLE IF EXISTS description;

CREATE TABLE name (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
	en TEXT NOT NULL,
	de TEXT,
    es TEXT,
    fr TEXT,
    ja TEXT,
    ko TEXT,
    ru TEXT,
    zh TEXT
);

CREATE TABLE description (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	en TEXT NOT NULL,
	de TEXT,
    es TEXT,
    fr TEXT,
    ja TEXT,
    ko TEXT,
    ru TEXT,
    zh TEXT
);

COMMIT;