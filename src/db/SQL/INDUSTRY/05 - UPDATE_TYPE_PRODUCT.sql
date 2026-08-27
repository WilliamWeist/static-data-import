BEGIN;

UPDATE entity_type
SET is_product = TRUE
WHERE entity_type.id IN (SELECT industry_product.product_id FROM industry_product);

COMMIT;