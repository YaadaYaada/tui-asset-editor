use bevy_reflect::Reflect;
use game_system::asset::asset_lib::AssetLib;
use ron;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Reflect)]
pub enum AuraType {
    Physical,
    Magic,
    Poison,
    None,
}

#[derive(Debug, Deserialize, Serialize)]
struct AuraRon {
    next_id: u32,
    defs: Vec<AuraDef>,
}

#[derive(Debug, Default)]
pub struct AuraLib {
    name_map: HashMap<String, usize>,
    id_map: HashMap<u32, usize>,
    pub defs: Vec<Arc<AuraDef>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Reflect)]
pub struct AuraDef {
    pub id: u32,
    pub name: String,
    pub icon: String,
    pub duration: f32,
    pub aura_type: AuraType,
    pub rules_text: String,
}

#[derive(Debug)]
pub struct Aura {
    pub def: Arc<AuraDef>,
}

impl FromStr for AuraType {
    type Err = ();

    fn from_str(input: &str) -> Result<AuraType, Self::Err> {
        match input {
            "Physical" => Ok(AuraType::Physical),
            "Magic" => Ok(AuraType::Magic),
            "Poison" => Ok(AuraType::Poison),
            "None" => Ok(AuraType::None),
            _ => Err(()),
        }
    }
}

impl fmt::Display for AuraType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AssetLib<AuraLib> for AuraLib {
    fn new(path: &str) -> Self {
        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let aura_ron: AuraRon = ron::from_str(&data).expect("RON was not well-formatted");

        let mut name_map = HashMap::new();
        let mut id_map = HashMap::new();
        let mut defs: Vec<Arc<AuraDef>> = vec![];
        for (i, def) in aura_ron.defs.into_iter().enumerate() {
            name_map.insert(def.name.clone(), i);
            id_map.insert(def.id, i);
            defs.push(Arc::new(def));
        }
        AuraLib {
            name_map,
            id_map,
            defs,
        }
    }
}

impl AuraLib {
    pub fn id(&self, id: u32) -> &AuraDef {
        &self.defs[self.id_map[&id]]
    }

    pub fn name(&self, name: String) -> &Arc<AuraDef> {
        &self.defs[self.name_map[&name]]
    }

    pub fn update_def(&mut self, def: Arc<AuraDef>) {
        let id = &def.id.clone();
        self.defs[self.id_map[&id]] = def
    }
}

impl Aura {
    pub fn new(def: &Arc<AuraDef>) -> Aura {
        Aura { def: def.clone() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::MECHANIC_TEST_DIR;

    #[test]
    fn auralib_load_and_access() {
        let aura_lib = AuraLib::new(&format!("{}/test/data/test_aura.ron", MECHANIC_TEST_DIR));
        let expected_aura_def = AuraDef {
            name: "Well Fed".to_string(),
            icon: "sprite/icon/cheese.png".to_string(),
            id: 0,
            aura_type: AuraType::None,
            duration: 60.0 * 60.0,
            rules_text: "You feel full! Your fortitudeness is through the roof.".to_string(),
        };
        assert_eq!(*aura_lib.id(0), expected_aura_def);
        assert_eq!(aura_lib.name("Shocked".to_string()).name, "Shocked");
    }

    #[test]
    fn aura_new() {
        let aura_lib = AuraLib::new(&format!("{}/test/data/test_aura.ron", MECHANIC_TEST_DIR));
        let _aura1 = Aura::new(aura_lib.name("Shocked".to_string()));
        let _aura2 = Aura::new(aura_lib.name("Shocked".to_string()));
    }
}
