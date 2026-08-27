BEGIN;

DROP TABLE IF EXISTS industry_activity;

CREATE TABLE industry_activity (
	id INTEGER PRIMARY KEY,
	name TEXT NOT NULL,
	description TEXT NOT NULL
);

COMMIT;