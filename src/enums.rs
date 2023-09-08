use std::{path::PathBuf, slice::Iter};

use bevy::reflect::Reflect;

#[derive(Copy, Clone, Debug, Reflect, Default, PartialEq)]
pub enum MapLayer {
    #[default]
    BASE,
    ACTIVATED,
    SELECTED,
    HOVERED,
}

impl MapLayer {
    pub fn iterator() -> Iter<'static, MapLayer> {
        static LAYERS: [MapLayer; 4] = [
            MapLayer::BASE,
            MapLayer::HOVERED,
            MapLayer::ACTIVATED,
            MapLayer::SELECTED,
        ];
        LAYERS.iter()
    }

    pub fn to_string(&self) -> String {
        match self {
            MapLayer::BASE => String::from("Base"),
            MapLayer::ACTIVATED => String::from("Activated"),
            MapLayer::SELECTED => String::from("Selected"),
            MapLayer::HOVERED => String::from("Hovered"),
        }
    }

    pub fn to_layer_level(&self) -> f32 {
        match self {
            MapLayer::BASE => 0.0,
            MapLayer::ACTIVATED => 1.0,
            MapLayer::SELECTED => 2.0,
            MapLayer::HOVERED => 3.0,
        }
    }

    pub fn from_id(id: u32) -> Option<MapLayer> {
        match id {
            0 => Some(MapLayer::BASE),
            1 => Some(MapLayer::ACTIVATED),
            2 => Some(MapLayer::SELECTED),
            3 => Some(MapLayer::HOVERED),
            _ => None,
        }
    }

    pub const fn to_id(self) -> u32 {
        match self {
            MapLayer::BASE => 0,
            MapLayer::ACTIVATED => 1,
            MapLayer::SELECTED => 2,
            MapLayer::HOVERED => 3,
        }
    }

    pub fn get_texture(self) -> PathBuf {
        let mut path_buf = PathBuf::new();
        let file_path = match self {
            MapLayer::BASE => String::from("grass-tile.png"),
            MapLayer::ACTIVATED => String::from("activated-tile.png"),
            MapLayer::SELECTED => String::from("selected-tile.png"),
            MapLayer::HOVERED => String::from("hovered-tile.png"),
        };
        path_buf.push(file_path);
        path_buf
    }
}
