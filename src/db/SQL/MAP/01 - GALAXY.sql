BEGIN;

DROP TABLE IF EXISTS map_galaxy;

CREATE TABLE map_galaxy (
	id INTEGER PRIMARY KEY,
	name_id INTEGER NOT NULL REFERENCES name(id),
	description_id INTEGER NOT NULL REFERENCES description(id)
);

COMMIT;