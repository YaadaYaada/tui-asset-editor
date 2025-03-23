use std::{fmt, fs::File, io::Read, str::FromStr, sync::Arc};

use bevy_reflect::Reflect;
use game_system::prelude::AssetLib;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::equipment::EquipmentDef;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Reflect)]
pub enum ItemType {
    Equipment,
    Consumable,
    Material,
    Miscellaneous,
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Reflect)]
pub enum ItemRarity {
    Junk,
    Common,
    Uncommon,
    Rare,
    Epic,
    Mythical,
}

impl fmt::Display for ItemRarity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ItemRon {
    next_id: u32,
    defs: Vec<ItemDef>,
}

#[derive(Debug, Default)]
pub struct ItemLib {
    name_map: HashMap<String, usize>,
    id_map: HashMap<u32, usize>,
    pub defs: Vec<Arc<ItemDef>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Reflect)]
pub struct ItemDef {
    pub id: u32,
    pub name: String,
    pub rules_text: String,
    pub flavor_text: String,
    pub icon: String,
    pub item_type: ItemType,
    pub item_rarity: ItemRarity,
    pub max_stack: u32,
    pub buy_value: u32,
    pub sell_value: u32,
    #[serde(default)]
    pub equipment_def: EquipmentDef,
}

#[derive(Debug)]
pub struct Item {
    pub def: Arc<ItemDef>,
    pub text: String,
}

impl FromStr for ItemType {
    type Err = ();

    fn from_str(input: &str) -> Result<ItemType, Self::Err> {
        match input {
            "Equipment" => Ok(ItemType::Equipment),
            "Consumable" => Ok(ItemType::Consumable),
            "Miscellaneous" => Ok(ItemType::Miscellaneous),
            _ => Err(()),
        }
    }
}

impl FromStr for ItemRarity {
    type Err = ();

    fn from_str(input: &str) -> Result<ItemRarity, Self::Err> {
        match input {
            "Junk" => Ok(ItemRarity::Junk),
            "Common" => Ok(ItemRarity::Common),
            "Uncommon" => Ok(ItemRarity::Uncommon),
            "Rare" => Ok(ItemRarity::Rare),
            "Epic" => Ok(ItemRarity::Epic),
            "Mythical" => Ok(ItemRarity::Mythical),
            _ => Err(()),
        }
    }
}

impl AssetLib<ItemLib> for ItemLib {
    fn new(path: &str) -> Self {
        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let item_ron: ItemRon = ron::from_str(&data).expect("RON was not well-formatted");

        let mut name_map = HashMap::new();
        let mut id_map = HashMap::new();
        let mut defs: Vec<Arc<ItemDef>> = vec![];
        for (i, def) in item_ron.defs.into_iter().enumerate() {
            name_map.insert(def.name.clone(), i);
            id_map.insert(def.id, i);
            defs.push(Arc::new(def));
        }
        ItemLib {
            name_map,
            id_map,
            defs,
        }
    }
}

impl ItemLib {
    pub fn id(&self, id: u32) -> &ItemDef {
        &self.defs[self.id_map[&id]]
    }

    pub fn update_def(&mut self, def: Arc<ItemDef>) {
        let id = &def.id.clone();
        self.defs[self.id_map[&id]] = def
    }

    pub fn name(&self, name: String) -> &Arc<ItemDef> {
        &self.defs[self.name_map[&name]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn item_new() {
        let item_def = ItemDef {
            name: "Cheddar Cheese".to_string(),
            rules_text: "Cheesed to meet you".to_string(),
            flavor_text: "Something quite flavorful".to_string(),
            icon: "sprite/icon/cheese.png".to_string(),
            id: 1,
            item_type: ItemType::Consumable,
            item_rarity: ItemRarity::Common,
            max_stack: 50,
            buy_value: 10,
            sell_value: 5,
            equipment_def: Default::default(),
        };
        let cheddar_cheese_def = Arc::new(item_def);
        let item_1 = Item {
            def: cheddar_cheese_def.clone(),
            text: String::new(),
        };

        let item_2 = Item {
            def: cheddar_cheese_def.clone(),
            text: String::new(),
        };

        assert_eq!(item_1.def.name, "Cheddar Cheese".to_string());
        assert_eq!(item_2.def.name, "Cheddar Cheese".to_string());
    }

    #[test]
    fn itemlib_load_and_access() {
        let item_lib = ItemLib::new(&format!("{}/test/data/test_item.ron", MECHANIC_TEST_DIR));
        let expected_item_def = ItemDef {
            name: "Red Potion".to_string(),
            rules_text: "".to_string(),
            flavor_text: "A vibrant red potion. Probably safe to drink.".to_string(),
            icon: "sprite/icon/red_potion.png".to_string(),
            id: 0,
            item_type: ItemType::Miscellaneous,
            item_rarity: ItemRarity::Common,
            max_stack: 50,
            buy_value: 10,
            sell_value: 5,
            equipment_def: Default::default(),
        };
        assert_eq!(*item_lib.id(0), expected_item_def);
        assert_eq!(item_lib.name("Red Potion".to_string()).name, "Red Potion");
    }
}
