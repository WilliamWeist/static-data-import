BEGIN;

DROP TABLE IF EXISTS entity_type;

CREATE TABLE entity_type (
	id INTEGER PRIMARY KEY,
    description_id INTEGER REFERENCES description(id),
    group_id INTEGER NOT NULL REFERENCES entity_group(id),
	name_id INTEGER NOT NULL REFERENCES name(id),
	published INTEGER NOT NULL CHECK (published IN (0, 1)),
    is_repackable INTEGER CHECK (is_repackable IN (0, 1)),
    packaged_volume REAL,
    volume REAL,
    is_product INTEGER CHECK (is_product IN (0, 1)) DEFAULT FALSE
);

COMMIT;