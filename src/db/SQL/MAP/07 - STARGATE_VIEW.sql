BEGIN;

DROP VIEW IF EXISTS v_stargate;

CREATE VIEW v_stargate AS 
SELECT
	map_stargate.solar_system_id AS from_system_id,
	from_system_name.en AS from_system_name,
	to_system_name.en AS to_system_name,
	map_stargate.destination_solar_system_id AS to_system_id
FROM map_stargate
INNER JOIN map_solar_system AS map_solar_from ON map_stargate.solar_system_id = map_solar_from.id
INNER JOIN name AS from_system_name ON map_solar_from.name_id = from_system_name.id
INNER JOIN map_solar_system AS map_solar_to ON map_stargate.destination_solar_system_id = map_solar_to.id
INNER JOIN name AS to_system_name ON map_solar_to.name_id = to_system_name.id;

COMMIT;