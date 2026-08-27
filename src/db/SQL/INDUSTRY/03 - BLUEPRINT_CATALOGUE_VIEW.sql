BEGIN;

DROP VIEW IF EXISTS v_blueprint_catalog;

CREATE VIEW v_blueprint_catalog AS
SELECT
    industry_blueprint.blueprint_id AS blueprint_id,
    blueprint_name.en AS blueprint_name,
    industry_activity.id AS activity_id,
    industry_activity.name AS activity_name,
    industry_product.product_id AS product_id,
    product_name.en AS product_name,
    industry_product.quantity AS quantity,
    industry_product.probability AS probability
FROM industry_blueprint
INNER JOIN industry_activity ON industry_blueprint.activity_id = industry_activity.id
INNER JOIN industry_product ON industry_product.blueprint_id = industry_blueprint.blueprint_id 
    AND industry_product.activity_id = industry_blueprint.activity_id
INNER JOIN entity_type AS blueprint_type ON industry_blueprint.blueprint_id = blueprint_type.id
INNER JOIN name AS blueprint_name ON blueprint_type.name_id = blueprint_name.id
INNER JOIN entity_type AS product_type ON industry_product.product_id = product_type.id
INNER JOIN name AS product_name ON product_type.name_id = product_name.id
WHERE blueprint_type.published = TRUE AND product_type.published = TRUE
ORDER BY blueprint_id, activity_id, product_id;

COMMIT;