use bevy::{prelude::*, utils::HashSet};
use hexx::Hex;

#[derive(Resource)]
pub struct TurnQueue {
    pub turn_number: i32,
}

#[derive(Resource)]
pub struct CursorPos(pub Vec2);

#[derive(Resource, Default)]
pub struct HexMap(pub HashSet<Hex>);

impl Default for TurnQueue {
    fn default() -> Self {
        TurnQueue { turn_number: 1 }
    }
}
