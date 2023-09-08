use bevy::prelude::*;
use hexx::{Direction, Hex};

use crate::enums::MapLayer;

#[derive(Component, Copy, Clone)]
pub struct Hovered;

#[derive(Component, Reflect)]
pub struct Unit {
    pub health: i32,
}

#[derive(Component, Reflect)]
pub struct MoveRange(pub u32);

#[derive(Component, Reflect, Default, Clone, Copy, Debug, PartialEq)]
#[reflect(Component)]
pub struct BoardLoc {
    pub hex: Hex,
}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct Activated;

#[derive(Component)]
pub struct BaseHex;

#[derive(Component, Reflect, Default, Clone, Copy, Debug)]
#[reflect(Component)]
pub struct HexTile(pub Hex);

#[derive(Component, Reflect, Default, Clone, Copy, Debug)]
#[reflect(Component)]
pub struct Layer {
    pub layer_type: MapLayer,
}

#[derive(Component, Reflect, Default, Clone, Copy, Debug)]
#[reflect(Component)]
pub struct MoveTarget(pub Hex);

#[derive(Component, PartialEq, Reflect, Default, Clone, Copy, Debug)]
#[reflect(Component)]
pub struct Moving {
    pub towards: Hex,
    pub direction: Direction,
}

#[derive(Component, Reflect, Default, Clone, Debug, PartialEq)]
#[reflect(Component)]
pub struct Path(pub Vec<Hex>);
