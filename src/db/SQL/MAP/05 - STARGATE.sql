BEGIN;

DROP TABLE IF EXISTS map_stargate;

CREATE TABLE map_stargate (
    id INTEGER PRIMARY KEY,
    solar_system_id INTEGER NOT NULL REFERENCES map_solar_system(id),
    position_id INTEGER NOT NULL REFERENCES map_position(id),
    destination_stargate_id INTEGER NOT NULL,
    destination_solar_system_id INTEGER NOT NULL REFERENCES map_solar_system(id)
);

COMMIT;