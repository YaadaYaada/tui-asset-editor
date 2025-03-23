use bevy_reflect::Reflect;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Reflect)]
pub enum AssetType {
    Aura,
    Item,
}

impl fmt::Display for AssetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait AssetLib<T> {
    fn new(path: &str) -> Self;
    fn save(&self, path: &str);
}
