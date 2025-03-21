use std::{fmt, str::FromStr};

use bevy_reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Reflect)]
pub enum EquipmentSlot {
    MainHand,
    OffHand,
    Head,
    Chest,
    Waist,
    Hands,
    Legs,
    Feet,
    Finger,
    Neck,
    Artifact,
    Accessory,
    None,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Reflect)]
pub struct EquipmentDef {
    pub slot: EquipmentSlot,
    pub armor: u32,
}

impl fmt::Display for EquipmentSlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EquipmentSlot::MainHand => write!(f, "MainHand"),
            EquipmentSlot::OffHand => write!(f, "OffHand"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl FromStr for EquipmentSlot {
    type Err = ();

    fn from_str(input: &str) -> Result<EquipmentSlot, Self::Err> {
        match input {
            "MainHand" => Ok(EquipmentSlot::MainHand),
            "OffHand" => Ok(EquipmentSlot::OffHand),
            "Head" => Ok(EquipmentSlot::Head),
            "Chest" => Ok(EquipmentSlot::Chest),
            "Waist" => Ok(EquipmentSlot::Waist),
            "Hands" => Ok(EquipmentSlot::Hands),
            "Legs" => Ok(EquipmentSlot::Legs),
            "Feet" => Ok(EquipmentSlot::Feet),
            "Finger" => Ok(EquipmentSlot::Finger),
            "Neck" => Ok(EquipmentSlot::Neck),
            "Artifact" => Ok(EquipmentSlot::Artifact),
            "Accessory" => Ok(EquipmentSlot::Accessory),
            "None" => Ok(EquipmentSlot::None),
            _ => Err(()),
        }
    }
}

impl Default for EquipmentDef {
    fn default() -> Self {
        EquipmentDef {
            slot: EquipmentSlot::None,
            armor: 0,
        }
    }
}
