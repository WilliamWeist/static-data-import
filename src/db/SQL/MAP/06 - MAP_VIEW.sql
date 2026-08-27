BEGIN;

DROP VIEW IF EXISTS v_map;

CREATE VIEW v_map AS 
SELECT 
	map_solar_system.id AS system_id,
	system_name.en AS system_name,
	map_constellation.id AS constellation_id,
	constellation_name.en AS constellation_name,
	map_region.id AS region_id,
	region_name.en AS region_name,
	map_solar_system.security_status,
	map_position.x AS x,
	map_position.y AS y,
	map_position.z AS z,
	galaxy_name.en AS galaxy_name
FROM map_solar_system
INNER JOIN name AS system_name ON map_solar_system.name_id = system_name.id
INNER JOIN map_constellation ON map_solar_system.constellation_id = map_constellation.id
INNER JOIN name AS constellation_name ON map_constellation.name_id = constellation_name.id
INNER JOIN map_region ON map_constellation.region_id = map_region.id
INNER JOIN name AS region_name ON map_region.name_id = region_name.id
INNER JOIN map_position ON map_solar_system.position_id = map_position.id
INNER JOIN map_galaxy ON map_region.galaxy_id = map_galaxy.id
INNER JOIN name AS galaxy_name ON map_galaxy.name_id = galaxy_name.id
ORDER BY region_id, constellation_id, system_id;

COMMIT;