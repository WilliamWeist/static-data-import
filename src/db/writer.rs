use std::{collections::HashMap, io::Write};

use anyhow::{Context, Result};
use rusqlite::{Connection, Statement, Transaction, params};

use crate::{
    config,
    sde::{
        self, Build, SDE,
        data::{
            Description, Name,
            entity::{category, group, r#type},
            industry::{activity, blueprint},
            map::{self, constellation, galaxy, region, solar_system, stargate},
        },
        types::ids,
    },
};

pub(crate) fn write_records(db: &mut Connection, sde: &SDE) -> Result<()> {
    write_galaxy_records(db, &sde.map.galaxies)
        .context("writing galaxy records")
        .inspect_err(|_| println!(" {}", config::ERROR_SYMBOL))?;
    write_region_records(db, &sde.map.regions)
        .context("writing region records")
        .inspect_err(|_| println!(" {}", config::ERROR_SYMBOL))?;
    write_constellation_records(db, &sde.map.constellations)
        .context("writing constellation records")
        .inspect_err(|_| println!(" {}", config::ERROR_SYMBOL))?;
    write_solar_system_records(db, &sde.map.solar_systems)
        .context("writing solar system records")
        .inspect_err(|_| println!(" {}", config::ERROR_SYMBOL))?;
    write_stargate_records(db, &sde.map.stargates)
        .context("writing stargate records")
        .inspect_err(|_| println!(" {}", config::ERROR_SYMBOL))?;

    write_category_records(db, &sde.entity.categories)
        .context("writing category records")
        .inspect_err(|_| println!(" {}", config::ERROR_SYMBOL))?;
    write_group_records(db, &sde.entity.groups)
        .context("writing group records")
        .inspect_err(|_| println!(" {}", config::ERROR_SYMBOL))?;
    write_type_records(db, &sde.entity.types)
        .context("writing type records")
        .inspect_err(|_| println!(" {}", config::ERROR_SYMBOL))?;

    write_activity_records(db, &sde.industry.activities)
        .context("writing activity records")
        .inspect_err(|_| println!(" {}", config::ERROR_SYMBOL))?;
    write_blueprint_records(db, &sde.industry.blueprints, &sde.industry.activities)
        .context("writing activity records")
        .inspect_err(|_| println!(" {}", config::ERROR_SYMBOL))?;
    update_type_records(db)
        .context("updating type records")
        .inspect_err(|_| println!(" {}", config::ERROR_SYMBOL))?;

    write_build_record(db, &sde.build)
        .context("writing SDE Build info")
        .inspect_err(|_| println!(" {}", config::ERROR_SYMBOL))?;

    Ok(())
}

fn write_build_record(db: &mut Connection, build: &Build) -> Result<()> {
    print!("\rWriting SDE Build info");
    std::io::stdout().flush().context("flushing stdout")?;
    let tx: Transaction<'_> = db.transaction()?;
    {
        let mut stmt: Statement<'_> =
            tx.prepare("INSERT INTO build (id, build_number, release_date) VALUES (?1, ?2, ?3)")?;
        stmt.execute(params![build.id, build.number, build.release_date])?;
    }
    tx.commit()?;
    println!(" {}", config::SUCCESS_SYMBOL);

    Ok(())
}

fn write_name_records(db: &mut Connection, names: &[Name]) -> Result<Vec<ids::NameID>> {
    let mut name_ids: Vec<ids::NameID> = Vec::new();
    let tx: Transaction<'_> = db.transaction()?;
    {
        let mut stmt: Statement<'_> = tx.prepare(
            "INSERT INTO name (en, de, es, fr, ja, ko, ru, zh) \
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8) RETURNING id",
        )?;
        for name in names {
            let id: ids::NameID = stmt.query_row(
                params![
                    name.en, name.de, name.es, name.fr, name.ja, name.ko, name.ru, name.zh
                ],
                |row| row.get(0),
            )?;
            name_ids.push(id);
        }
    }
    tx.commit()?;

    Ok(name_ids)
}

fn write_description_records(
    db: &mut Connection,
    descriptions: &[Description],
) -> Result<Vec<ids::DescriptionID>> {
    let mut description_ids: Vec<ids::DescriptionID> = Vec::new();
    let tx: Transaction<'_> = db.transaction()?;
    {
        let mut stmt: Statement<'_> = tx.prepare(
            "INSERT INTO description (en, de, es, fr, ja, ko, ru, zh) \
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8) RETURNING id",
        )?;
        for description in descriptions {
            let id: ids::DescriptionID = stmt.query_row(
                params![
                    description.en,
                    description.de,
                    description.es,
                    description.fr,
                    description.ja,
                    description.ko,
                    description.ru,
                    description.zh
                ],
                |row| row.get(0),
            )?;
            description_ids.push(id);
        }
    }
    tx.commit()?;

    Ok(description_ids)
}

fn write_position_records(
    db: &mut Connection,
    positions: &[map::Position],
) -> Result<Vec<ids::PositionID>> {
    let mut position_ids: Vec<ids::PositionID> = Vec::new();
    let tx: Transaction<'_> = db.transaction()?;
    {
        let mut stmt: Statement<'_> =
            tx.prepare("INSERT INTO map_position (x, y, z) VALUES (?1, ?2, ?3) RETURNING id")?;
        for position in positions {
            let id: ids::PositionID = stmt
                .query_row(params![position.x, position.y, position.z], |row| {
                    row.get(0)
                })?;
            position_ids.push(id);
        }
    }
    tx.commit()?;

    Ok(position_ids)
}

fn write_position_2d_records(
    db: &mut Connection,
    positions_2d: &[map::Position2d],
) -> Result<Vec<ids::Position2dID>> {
    let mut position_2d_ids: Vec<ids::Position2dID> = Vec::new();
    let tx: Transaction<'_> = db.transaction()?;
    {
        let mut stmt: Statement<'_> =
            tx.prepare("INSERT INTO map_position_2d (x, y) VALUES (?1, ?2) RETURNING id")?;
        for position_2d in positions_2d {
            let id: ids::Position2dID =
                stmt.query_row(params![position_2d.x, position_2d.y], |row| row.get(0))?;
            position_2d_ids.push(id);
        }
    }
    tx.commit()?;

    Ok(position_2d_ids)
}

fn write_galaxy_records(db: &mut Connection, galaxies: &[galaxy::Galaxy]) -> Result<()> {
    print!("\rWriting Galaxy records");
    std::io::stdout().flush().context("flushing stdout")?;
    let mut names: Vec<Name> = Vec::new();
    let name_ids: Vec<ids::NameID>;
    let mut descriptions: Vec<Description> = Vec::new();
    let description_ids: Vec<ids::DescriptionID>;

    for galaxy in galaxies {
        names.push(galaxy.name.clone());
        descriptions.push(galaxy.description.clone());
    }
    name_ids = write_name_records(db, &names).context("writing name records")?;
    description_ids =
        write_description_records(db, &descriptions).context("writing description records")?;

    let tx: Transaction<'_> = db.transaction()?;
    {
        let mut stmt: Statement<'_> = tx.prepare(
            "INSERT INTO map_galaxy (id, name_id, description_id) \
            VALUES (?1, ?2, ?3)",
        )?;
        let mut index: usize = 0;
        for galaxy in galaxies {
            stmt.execute(params![galaxy.id, name_ids[index], description_ids[index]])?;
            index += 1;
        }
    }
    tx.commit()?;
    println!(" {}", config::SUCCESS_SYMBOL);

    Ok(())
}

fn write_region_records(db: &mut Connection, regions: &[region::Region]) -> Result<()> {
    print!("\rWriting Region records");
    std::io::stdout().flush().context("flushing stdout")?;
    let mut names: Vec<Name> = Vec::new();
    let name_ids: Vec<ids::NameID>;
    let mut descriptions: Vec<Description> = Vec::new();
    let description_ids: Vec<ids::DescriptionID>;
    let mut positions: Vec<map::Position> = Vec::new();
    let position_ids: Vec<ids::PositionID>;

    for region in regions {
        names.push(region.name.clone());
        if let Some(description) = &region.description {
            descriptions.push(description.clone());
        }
        positions.push(region.position.clone());
    }

    name_ids = write_name_records(db, &names).context("writing name records")?;
    description_ids =
        write_description_records(db, &descriptions).context("writing description records")?;
    position_ids = write_position_records(db, &positions).context("writing position records")?;

    let tx: Transaction<'_> = db.transaction()?;
    {
        let mut stmt: Statement<'_> = tx.prepare(
            "INSERT INTO map_region (id, name_id, description_id, galaxy_id, position_id) \
            VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;
        let mut index: usize = 0;
        let mut description_index: usize = 0;
        for region in regions {
            let description: Option<ids::DescriptionID>;
            if region.description.is_none() {
                description = None;
            } else {
                description = Some(description_ids[description_index]);
                description_index += 1;
            }
            let galaxy_id: ids::GalaxyID = galaxy::get_galaxy_id(region.id)?;
            stmt.execute(params![
                region.id,
                name_ids[index],
                description,
                galaxy_id,
                position_ids[index]
            ])?;
            index += 1;
        }
    }
    tx.commit()?;
    println!(" {}", config::SUCCESS_SYMBOL);

    Ok(())
}

fn write_constellation_records(
    db: &mut Connection,
    constellations: &[constellation::Constellation],
) -> Result<()> {
    print!("\rWriting Constellation records");
    std::io::stdout().flush().context("flushing stdout")?;
    let mut names: Vec<Name> = Vec::new();
    let name_ids: Vec<ids::NameID>;
    let mut positions: Vec<map::Position> = Vec::new();
    let position_ids: Vec<ids::PositionID>;

    for constellation in constellations {
        names.push(constellation.name.clone());
        positions.push(constellation.position.clone());
    }

    name_ids = write_name_records(db, &names).context("writing name records")?;
    position_ids = write_position_records(db, &positions).context("writing position records")?;

    let tx: Transaction<'_> = db.transaction()?;
    {
        let mut stmt: Statement<'_> = tx.prepare(
            "INSERT INTO map_constellation (id, name_id, region_id, position_id) \
            VALUES (?1, ?2, ?3, ?4)",
        )?;
        let mut index: usize = 0;
        for constellation in constellations {
            stmt.execute(params![
                constellation.id,
                name_ids[index],
                constellation.region_id,
                position_ids[index]
            ])?;
            index += 1;
        }
    }
    tx.commit()?;
    println!(" {}", config::SUCCESS_SYMBOL);

    Ok(())
}

fn write_solar_system_records(
    db: &mut Connection,
    solar_systems: &[solar_system::SolarSystem],
) -> Result<()> {
    print!("\rWriting Solar System records");
    std::io::stdout().flush().context("flushing stdout")?;
    let mut names: Vec<Name> = Vec::new();
    let name_ids: Vec<ids::NameID>;
    let mut positions: Vec<map::Position> = Vec::new();
    let position_ids: Vec<ids::PositionID>;
    let mut positions_2d: Vec<map::Position2d> = Vec::new();
    let position_2d_ids: Vec<ids::Position2dID>;

    for solar_systems in solar_systems {
        names.push(solar_systems.name.clone());
        positions.push(solar_systems.position.clone());
        if let Some(position_2d) = &solar_systems.position_2d {
            positions_2d.push(position_2d.clone());
        }
    }

    name_ids = write_name_records(db, &names).context("writing name records")?;
    position_ids = write_position_records(db, &positions).context("writing position records")?;
    position_2d_ids =
        write_position_2d_records(db, &positions_2d).context("writing position2d records")?;

    let tx: Transaction<'_> = db.transaction()?;
    {
        let mut stmt: Statement<'_> = tx.prepare(
            "INSERT INTO map_solar_system \
            (id, name_id, constellation_id, security_status, position_id, position_2d_id) \
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )?;
        let mut index: usize = 0;
        let mut position_2d_index: usize = 0;
        for solar_system in solar_systems {
            let position_2d: Option<ids::Position2dID>;
            if solar_system.position_2d.is_none() {
                position_2d = None;
            } else {
                position_2d = Some(position_2d_ids[position_2d_index]);
                position_2d_index += 1;
            }
            stmt.execute(params![
                solar_system.id,
                name_ids[index],
                solar_system.constellation_id,
                solar_system.security_status,
                position_ids[index],
                position_2d
            ])?;
            index += 1;
        }
    }
    tx.commit()?;
    println!(" {}", config::SUCCESS_SYMBOL);

    Ok(())
}

fn write_stargate_records(db: &mut Connection, stargates: &[stargate::Stargate]) -> Result<()> {
    print!("\rWriting Stargate records");
    std::io::stdout().flush().context("flushing stdout")?;
    let mut positions: Vec<map::Position> = Vec::new();
    let position_ids: Vec<ids::PositionID>;

    for stargate in stargates {
        positions.push(stargate.position.clone());
    }

    position_ids = write_position_records(db, &positions).context("writing position records")?;

    let tx: Transaction<'_> = db.transaction()?;
    {
        let mut stmt: Statement<'_> = tx.prepare(
            "INSERT INTO map_stargate \
            (id, solar_system_id, position_id, destination_stargate_id, destination_solar_system_id) \
            VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;
        let mut index: usize = 0;
        for stargate in stargates {
            stmt.execute(params![
                stargate.id,
                stargate.solar_system_id,
                position_ids[index],
                stargate.destination.stargate_id,
                stargate.destination.solar_system_id
            ])?;
            index += 1;
        }
    }
    tx.commit()?;
    println!(" {}", config::SUCCESS_SYMBOL);

    Ok(())
}

fn write_category_records(db: &mut Connection, categories: &[category::Category]) -> Result<()> {
    print!("\rWriting Category records");
    std::io::stdout().flush().context("flushing stdout")?;
    let mut names: Vec<Name> = Vec::new();
    let name_ids: Vec<ids::NameID>;

    for category in categories {
        names.push(category.name.clone());
    }

    name_ids = write_name_records(db, &names).context("writing name records")?;

    let tx: Transaction<'_> = db.transaction()?;
    {
        let mut stmt: Statement<'_> =
            tx.prepare("INSERT INTO entity_category (id, name_id, published) VALUES (?1, ?2, ?3)")?;
        let mut index: usize = 0;
        for category in categories {
            stmt.execute(params![category.id, name_ids[index], category.published])?;
            index += 1;
        }
    }
    tx.commit()?;
    println!(" {}", config::SUCCESS_SYMBOL);

    Ok(())
}

fn write_group_records(db: &mut Connection, groups: &[group::Group]) -> Result<()> {
    print!("\rWriting Group records");
    std::io::stdout().flush().context("flushing stdout")?;
    let mut names: Vec<Name> = Vec::new();
    let name_ids: Vec<ids::NameID>;

    for group in groups {
        names.push(group.name.clone());
    }

    name_ids = write_name_records(db, &names).context("writing name records")?;

    let tx: Transaction<'_> = db.transaction()?;
    {
        let mut stmt: Statement<'_> = tx.prepare(
            "INSERT INTO entity_group (id, category_id, name_id, published) \
            VALUES (?1, ?2, ?3, ?4)",
        )?;
        let mut index: usize = 0;
        for group in groups {
            stmt.execute(params![
                group.id,
                group.category_id,
                name_ids[index],
                group.published
            ])?;
            index += 1;
        }
    }
    tx.commit()?;
    println!(" {}", config::SUCCESS_SYMBOL);

    Ok(())
}

fn write_type_records(db: &mut Connection, types: &[r#type::Type]) -> Result<()> {
    print!("\rWriting Type records");
    std::io::stdout().flush().context("flushing stdout")?;
    let mut descriptions: Vec<Description> = Vec::new();
    let description_ids: Vec<ids::DescriptionID>;
    let mut names: Vec<Name> = Vec::new();
    let name_ids: Vec<ids::NameID>;

    for r#type in types {
        if let Some(description) = &r#type.description {
            descriptions.push(description.clone());
        }
        names.push(r#type.name.clone());
    }

    description_ids =
        write_description_records(db, &descriptions).context("writing description records")?;
    name_ids = write_name_records(db, &names).context("writing name records")?;

    let tx: Transaction<'_> = db.transaction()?;
    {
        let mut stmt: Statement<'_> = tx.prepare(
            "INSERT INTO entity_type \
            (id, description_id, group_id, name_id, published, is_repackable, packaged_volume, volume) \
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )?;
        let mut index: usize = 0;
        let mut description_index: usize = 0;
        for r#type in types {
            let description_id: Option<ids::DescriptionID>;
            if r#type.description.is_none() {
                description_id = None;
            } else {
                description_id = Some(description_ids[description_index]);
                description_index += 1;
            }
            stmt.execute(params![
                r#type.id,
                description_id,
                r#type.group_id,
                name_ids[index],
                r#type.published,
                r#type.is_repackable,
                r#type.packaged_volume,
                r#type.volume
            ])?;
            index += 1;
        }
    }
    tx.commit()?;
    println!(" {}", config::SUCCESS_SYMBOL);

    Ok(())
}

fn write_activity_records(db: &mut Connection, activities: &[activity::Activity]) -> Result<()> {
    print!("\rWriting Activity records");
    std::io::stdout().flush().context("flushing stdout")?;

    let tx: Transaction<'_> = db.transaction()?;
    {
        let mut stmt: Statement<'_> = tx
            .prepare("INSERT INTO industry_activity (id, name, description) VALUES (?1, ?2, ?3)")?;
        for activitie in activities {
            stmt.execute(params![activitie.id, activitie.name, activitie.description])?;
        }
    }
    tx.commit()?;
    println!(" {}", config::SUCCESS_SYMBOL);

    Ok(())
}

fn write_blueprint_records(
    db: &mut Connection,
    blueprints: &[blueprint::Blueprint],
    activities: &[activity::Activity],
) -> Result<()> {
    print!("\rWriting Blueprint records");
    std::io::stdout().flush().context("flushing stdout")?;
    let mut activities_map: HashMap<String, ids::IndustryActivityID> = HashMap::new();
    for activity in activities {
        activities_map.insert(activity.name.clone(), activity.id.clone());
    }
    let mut materials_records: Vec<(
        &ids::TypeID,
        &ids::IndustryActivityID,
        &Vec<blueprint::Material>,
    )> = Vec::new();
    let mut products_records: Vec<(
        &ids::TypeID,
        &ids::IndustryActivityID,
        &Vec<blueprint::Product>,
    )> = Vec::new();
    let mut skills_records: Vec<(
        &ids::TypeID,
        &ids::IndustryActivityID,
        &Vec<blueprint::Skill>,
    )> = Vec::new();

    let tx: Transaction<'_> = db.transaction()?;
    {
        let mut stmt: Statement<'_> = tx.prepare(
            "INSERT INTO industry_blueprint \
            (blueprint_id, activity_id, time, max_production_limit) \
            VALUES (?1, ?2, ?3, ?4)",
        )?;
        for blueprint in blueprints {
            if let Some(copying) = &blueprint.activities.copying {
                let activity_id: &ids::IndustryActivityID = activities_map
                    .get("Copying")
                    .context("retrieving Copying id")?;
                stmt.execute(params![
                    blueprint.blueprint_type_id,
                    activity_id,
                    copying.time,
                    blueprint.max_production_limit
                ])?;
                if let Some(materials) = &copying.materials {
                    materials_records.push((&blueprint.blueprint_type_id, activity_id, materials));
                }
                if let Some(skills) = &copying.skills {
                    skills_records.push((&blueprint.blueprint_type_id, activity_id, skills));
                }
            }

            if let Some(invention) = &blueprint.activities.invention {
                let activity_id: &ids::IndustryActivityID = activities_map
                    .get("Invention")
                    .context("retrieving Invention id")?;
                stmt.execute(params![
                    blueprint.blueprint_type_id,
                    activity_id,
                    invention.time,
                    blueprint.max_production_limit
                ])?;
                if let Some(materials) = &invention.materials {
                    materials_records.push((&blueprint.blueprint_type_id, activity_id, materials));
                }
                if let Some(products) = &invention.products {
                    products_records.push((&blueprint.blueprint_type_id, activity_id, products));
                }
                if let Some(skills) = &invention.skills {
                    skills_records.push((&blueprint.blueprint_type_id, activity_id, skills));
                }
            }

            if let Some(manufacturing) = &blueprint.activities.manufacturing {
                let activity_id: &ids::IndustryActivityID = activities_map
                    .get("Manufacturing")
                    .context("retrieving Manufacturing id")?;
                stmt.execute(params![
                    blueprint.blueprint_type_id,
                    activity_id,
                    manufacturing.time,
                    blueprint.max_production_limit
                ])?;
                if let Some(materials) = &manufacturing.materials {
                    materials_records.push((&blueprint.blueprint_type_id, activity_id, materials));
                }
                if let Some(products) = &manufacturing.products {
                    products_records.push((&blueprint.blueprint_type_id, activity_id, products));
                }
                if let Some(skills) = &manufacturing.skills {
                    skills_records.push((&blueprint.blueprint_type_id, activity_id, skills));
                }
            }

            if let Some(reaction) = &blueprint.activities.reaction {
                let activity_id: &ids::IndustryActivityID = activities_map
                    .get("Reactions")
                    .context("retrieving Reactions id")?;
                stmt.execute(params![
                    blueprint.blueprint_type_id,
                    activity_id,
                    reaction.time,
                    blueprint.max_production_limit
                ])?;
                if let Some(materials) = &reaction.materials {
                    materials_records.push((&blueprint.blueprint_type_id, activity_id, materials));
                }
                if let Some(products) = &reaction.products {
                    products_records.push((&blueprint.blueprint_type_id, activity_id, products));
                }
                if let Some(skills) = &reaction.skills {
                    skills_records.push((&blueprint.blueprint_type_id, activity_id, skills));
                }
            }

            if let Some(research_material) = &blueprint.activities.research_material {
                let activity_id: &ids::IndustryActivityID = activities_map
                    .get("Material Efficiency Research")
                    .context("retrieving Material Efficiency Research id")?;
                stmt.execute(params![
                    blueprint.blueprint_type_id,
                    activity_id,
                    research_material.time,
                    blueprint.max_production_limit
                ])?;
                if let Some(materials) = &research_material.materials {
                    materials_records.push((&blueprint.blueprint_type_id, activity_id, materials));
                }
                if let Some(skills) = &research_material.skills {
                    skills_records.push((&blueprint.blueprint_type_id, activity_id, skills));
                }
            }

            if let Some(research_time) = &blueprint.activities.research_time {
                let activity_id: &ids::IndustryActivityID = activities_map
                    .get("Time Efficiency Research")
                    .context("retrieving Time Efficiency Research id")?;
                stmt.execute(params![
                    blueprint.blueprint_type_id,
                    activity_id,
                    research_time.time,
                    blueprint.max_production_limit
                ])?;
                if let Some(materials) = &research_time.materials {
                    materials_records.push((&blueprint.blueprint_type_id, activity_id, materials));
                }
                if let Some(skills) = &research_time.skills {
                    skills_records.push((&blueprint.blueprint_type_id, activity_id, skills));
                }
            }
        }
    }
    tx.commit()?;
    println!(" {}", config::SUCCESS_SYMBOL);
    write_blueprint_materials_records(db, &materials_records)
        .context("writing blueprint material records")?;
    write_blueprint_products_records(db, &products_records)
        .context("writing blueprint product records")?;
    write_blueprint_skills_records(db, &skills_records)
        .context("writing blueprint product records")?;

    Ok(())
}

fn write_blueprint_materials_records(
    db: &mut Connection,
    materials_records: &[(
        &ids::TypeID,
        &ids::IndustryActivityID,
        &Vec<blueprint::Material>,
    )],
) -> Result<()> {
    print!("\rWriting Blueprint Materials records");
    std::io::stdout().flush().context("flushing stdout")?;

    let tx: Transaction<'_> = db.transaction()?;
    {
        let mut stmt: Statement<'_> = tx.prepare(
            "INSERT INTO industry_material \
            (blueprint_id, activity_id, material_id, quantity) \
            VALUES (?1, ?2, ?3, ?4)",
        )?;
        for (blueprint_id, activity_id, materials) in materials_records {
            if sde::quirks::BROKEN_MATERIAL_BLUEPRINT_IDS.contains(blueprint_id) {
                continue;
            }
            for material in *materials {
                stmt.execute(params![
                    blueprint_id,
                    activity_id,
                    material.type_id,
                    material.quantity
                ])?;
            }
        }
    }
    tx.commit()?;
    println!(" {}", config::SUCCESS_SYMBOL);

    Ok(())
}

fn write_blueprint_products_records(
    db: &mut Connection,
    products_records: &[(
        &ids::TypeID,
        &ids::IndustryActivityID,
        &Vec<blueprint::Product>,
    )],
) -> Result<()> {
    print!("\rWriting Blueprint Products records");
    std::io::stdout().flush().context("flushing stdout")?;

    let tx: Transaction<'_> = db.transaction()?;
    {
        let mut stmt: Statement<'_> = tx.prepare(
            "INSERT INTO industry_product \
            (blueprint_id, activity_id, product_id, quantity, probability) \
            VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;
        for (blueprint_id, activity_id, products) in products_records {
            if sde::quirks::BROKEN_PRODUCT_BLUEPRINT_IDS.contains(blueprint_id) {
                continue;
            }
            for product in *products {
                stmt.execute(params![
                    blueprint_id,
                    activity_id,
                    product.type_id,
                    product.quantity,
                    product.probability
                ])?;
            }
        }
    }
    tx.commit()?;
    println!(" {}", config::SUCCESS_SYMBOL);

    Ok(())
}

fn write_blueprint_skills_records(
    db: &mut Connection,
    skills_records: &[(
        &ids::TypeID,
        &ids::IndustryActivityID,
        &Vec<blueprint::Skill>,
    )],
) -> Result<()> {
    print!("\rWriting Blueprint Skills records");
    std::io::stdout().flush().context("flushing stdout")?;

    let tx: Transaction<'_> = db.transaction()?;
    {
        let mut stmt: Statement<'_> = tx.prepare(
            "INSERT INTO industry_skill \
            (blueprint_id, activity_id, skill_id, level) \
            VALUES (?1, ?2, ?3, ?4)",
        )?;
        for (blueprint_id, activity_id, skills) in skills_records {
            for skill in *skills {
                stmt.execute(params![
                    blueprint_id,
                    activity_id,
                    skill.type_id,
                    skill.level
                ])?;
            }
        }
    }
    tx.commit()?;
    println!(" {}", config::SUCCESS_SYMBOL);

    Ok(())
}

fn update_type_records(db: &mut Connection) -> Result<()> {
    print!("\rUpdating Type records");
    std::io::stdout().flush().context("flushing stdout")?;

    db.execute_batch(include_str!("SQL/INDUSTRY/05 - UPDATE_TYPE_PRODUCT.sql"))?;
    println!(" {}", config::SUCCESS_SYMBOL);

    Ok(())
}
