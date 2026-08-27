BEGIN;

DROP TABLE IF EXISTS map_solar_system;

CREATE TABLE map_solar_system (
    id INTEGER PRIMARY KEY,
	name_id INTEGER NOT NULL REFERENCES name(id),
    constellation_id INTEGER NOT NULL REFERENCES map_constellation(id),
    security_status REAL NOT NULL,
    position_id INTEGER NOT NULL REFERENCES map_position(id),
    position_2d_id INTEGER REFERENCES map_position_2d(id)
);

COMMIT;
