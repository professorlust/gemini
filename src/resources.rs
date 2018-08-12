use serde::de::Deserialize;
use serde_json;
use std::{collections::HashMap, str};

use astronomicals::planet::PlanetEconomy;
use economy::Commodity;
use economy::Schematic;
use entities::Faction;
use ship::ShipCharacteristics;

/// Generic Resource trait to be implemented by all resource types which should
/// be loaded at compile time.
/// KEY must be unique to the specific resource (e.g the filename of the
/// resource).
pub trait Resource: Deserialize<'static> {
    const KEY: &'static str;
}

lazy_static! {
    // Load resources at compile time.
    // TODO: Convert to resource at compile time to save resources.
    static ref RESOURCES: HashMap<&'static str, &'static str> = {
        let mut res = HashMap::new();
        res.insert(
            AstronomicalNamesResource::KEY,
            include_str!("../res/astronomical_names.json"),
        );
        res.insert(
            ShipResource::KEY,
            include_str!("../res/ships.json"),
        );
        res.insert(
            SchematicResource::KEY,
            include_str!("../res/schematics.json"),
        );
        res
    };
}

/// Attempts to returns the resource with the given type, will return None
/// if the type has no resource or if the deserialization fails.
pub fn fetch_resource<T: Resource>() -> Option<T> {
    let res_str = RESOURCES.get(T::KEY).unwrap();
    match serde_json::from_str(res_str) {
        Ok(res) => Some(res),
        Err(msg) => {
            error!("{}", msg);
            None
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
/// Resource used in name generation of celestial objects.
pub struct AstronomicalNamesResource {
    pub names: Vec<String>,
    pub scientific_names: Vec<String>,
    pub greek: Vec<String>,
    pub roman: Vec<String>,
    pub decorators: Vec<String>,
}

impl Resource for AstronomicalNamesResource {
    const KEY: &'static str = "astronomical_names";
}

#[derive(Serialize, Deserialize, Debug)]
/// Resource with all ships available in the game.
pub struct ShipResource {
    pub ships: Vec<ShipCharacteristics>,
}

impl Resource for ShipResource {
    const KEY: &'static str = "ships";
}

#[derive(Serialize, Deserialize, Debug)]
/// Resource containing all schematics.
pub struct SchematicResource {
    pub schematics: Vec<Schematic>,
}

impl Resource for SchematicResource {
    const KEY: &'static str = "schematics";
}
