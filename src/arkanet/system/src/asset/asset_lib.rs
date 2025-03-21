use bevy_reflect::Reflect;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Reflect)]
pub enum AssetType {
    Aura,
    Character,
    Item,
}

impl fmt::Display for AssetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait AssetLib<T> {
    fn new(ron_date: &str) -> Self;
}
