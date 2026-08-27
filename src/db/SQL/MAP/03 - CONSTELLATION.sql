BEGIN;

DROP TABLE IF EXISTS map_constellation;

CREATE TABLE map_constellation (
	id INTEGER PRIMARY KEY,
	name_id INTEGER NOT NULL REFERENCES name(id),
    region_id INTEGER NOT NULL REFERENCES map_region(id),
    position_id INTEGER NOT NULL REFERENCES map_position(id)
);

COMMIT;