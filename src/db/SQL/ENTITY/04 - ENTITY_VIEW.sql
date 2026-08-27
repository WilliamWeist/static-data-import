BEGIN;

DROP VIEW IF EXISTS v_entity;

CREATE VIEW v_entity AS 
SELECT 
	entity_type.id AS type_id,
	type_name.en AS type_name,
	entity_group.id AS group_id,
	group_name.en AS group_name,
	entity_category.id AS category_id,
	category_name.en AS category_name,
	entity_type.is_product AS is_product
FROM entity_type
INNER JOIN name AS type_name ON entity_type.name_id = type_name.id
INNER JOIN entity_group ON entity_type.group_id = entity_group.id
INNER JOIN name AS group_name ON entity_group.name_id = group_name.id
INNER JOIN entity_category ON entity_group.category_id = entity_category.id
INNER JOIN name AS category_name ON entity_category.name_id = category_name.id
WHERE entity_type.published = TRUE
ORDER BY entity_type.id;

COMMIT;