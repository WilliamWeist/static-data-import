BEGIN;

DROP TABLE IF EXISTS industry_blueprint;
DROP TABLE IF EXISTS industry_material;
DROP TABLE IF EXISTS industry_product;

CREATE TABLE industry_blueprint (
	blueprint_id INTEGER NOT NULL REFERENCES entity_type(id),
    activity_id INTEGER NOT NULL REFERENCES industry_activity(id),
    time INTEGER NOT NULL,
	max_production_limit INTEGER NOT NULL,
    PRIMARY KEY (blueprint_id, activity_id)
);

CREATE TABLE industry_material (
	blueprint_id INTEGER NOT NULL,
    activity_id INTEGER NOT NULL,
    material_id INTEGER NOT NULL REFERENCES entity_type(id),
	quantity INTEGER NOT NULL,
    FOREIGN KEY (blueprint_id, activity_id) REFERENCES industry_blueprint(blueprint_id, activity_id)
);

CREATE TABLE industry_product (
	blueprint_id INTEGER NOT NULL,
    activity_id INTEGER NOT NULL,
    product_id INTEGER NOT NULL REFERENCES entity_type(id),
	quantity INTEGER NOT NULL,
    probability REAL,
    FOREIGN KEY (blueprint_id, activity_id) REFERENCES industry_blueprint(blueprint_id, activity_id)

);

CREATE TABLE industry_skill (
	blueprint_id INTEGER NOT NULL,
    activity_id INTEGER NOT NULL,
    skill_id INTEGER NOT NULL REFERENCES entity_type(id),
	level INTEGER NOT NULL,
    FOREIGN KEY (blueprint_id, activity_id) REFERENCES industry_blueprint(blueprint_id, activity_id)

);


COMMIT;