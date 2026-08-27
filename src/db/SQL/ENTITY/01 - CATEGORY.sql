BEGIN;

DROP TABLE IF EXISTS entity_category;

CREATE TABLE entity_category (
	id INTEGER PRIMARY KEY,
	name_id INTEGER NOT NULL REFERENCES name(id),
	published INTEGER NOT NULL CHECK (published IN (0, 1))
);

COMMIT;