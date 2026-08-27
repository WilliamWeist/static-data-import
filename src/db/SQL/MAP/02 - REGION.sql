BEGIN;

DROP TABLE IF EXISTS map_region;

CREATE TABLE map_region (
	id INTEGER PRIMARY KEY,
	name_id INTEGER NOT NULL REFERENCES name(id),
	description_id INTEGER REFERENCES description(id),
    galaxy_id INTEGER NOT NULL REFERENCES map_galaxy(id),
    position_id INTEGER NOT NULL REFERENCES map_position(id)
);

COMMIT;