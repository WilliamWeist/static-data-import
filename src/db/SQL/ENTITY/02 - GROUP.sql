BEGIN;

DROP TABLE IF EXISTS entity_group;

CREATE TABLE entity_group (
	id INTEGER PRIMARY KEY,
    category_id INTEGER NOT NULL REFERENCES entity_category(id),
	name_id INTEGER NOT NULL REFERENCES name(id),
	published INTEGER NOT NULL CHECK (published IN (0, 1))
);

COMMIT;