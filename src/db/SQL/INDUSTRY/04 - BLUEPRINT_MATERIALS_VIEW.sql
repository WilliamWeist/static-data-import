BEGIN;

DROP VIEW IF EXISTS v_blueprint_materials;

CREATE VIEW v_blueprint_materials AS
SELECT
	industry_blueprint.activity_id AS activity_id,
	industry_activity.name AS activity_name,
	industry_material.blueprint_id AS blueprint_id,
	blueprint_name.en AS blueprint_name,
	industry_material.material_id AS material_id,
	material_name.en AS material_name,
	industry_material.quantity AS quantity,
	material_type.group_id AS group_id,
	group_name.en AS group_name,
	entity_category.id AS category_id,
	category_name.en AS category_name
FROM industry_material
INNER JOIN industry_blueprint ON industry_material.blueprint_id = industry_blueprint.blueprint_id 
    AND industry_material.activity_id = industry_blueprint.activity_id
INNER JOIN industry_activity ON industry_blueprint.activity_id = industry_activity.id
INNER JOIN entity_type AS blueprint_type ON industry_blueprint.blueprint_id = blueprint_type.id
INNER JOIN name AS blueprint_name ON blueprint_type.name_id = blueprint_name.id
INNER JOIN entity_type AS material_type ON industry_material.material_id = material_type.id
INNER JOIN name AS material_name ON material_type.name_id = material_name.id
INNER JOIN entity_group ON material_type.group_id = entity_group.id
INNER JOIN name AS group_name ON entity_group.name_id = group_name.id
INNER JOIN entity_category ON entity_group.category_id = entity_category.id
INNER JOIN name AS category_name ON entity_category.name_id = category_name.id
WHERE blueprint_type.published = TRUE
ORDER BY blueprint_id, activity_id, category_id, group_id DESC, material_id;

COMMIT;