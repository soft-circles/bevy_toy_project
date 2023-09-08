use bevy::prelude::*;
use hexx::Hex;

use crate::{
    components::{Activated, BaseHex, HexTile, Hovered, Layer, Selected},
    constants::LAYOUT,
    enums::MapLayer,
};

pub struct LayersPlugin;

impl Plugin for LayersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                tile_in_layer_added::<Selected, { MapLayer::to_id(MapLayer::SELECTED) }>,
                tile_in_layer_removed::<Selected, { MapLayer::to_id(MapLayer::SELECTED) }>,
                tile_in_layer_added::<Hovered, { MapLayer::to_id(MapLayer::HOVERED) }>,
                tile_in_layer_removed::<Hovered, { MapLayer::to_id(MapLayer::HOVERED) }>,
                tile_in_layer_added::<Activated, { MapLayer::to_id(MapLayer::ACTIVATED) }>,
                tile_in_layer_removed::<Activated, { MapLayer::to_id(MapLayer::ACTIVATED) }>,
            ),
        );
    }
}

fn spawn_tiles(
    commands: &mut Commands,
    layer_entity: &Entity,
    layer: &Layer,
    hexes: Vec<Hex>,
    asset: &Handle<Image>,
) {
    let children = hexes
        .iter()
        .map(|_| commands.spawn_empty().id())
        .collect::<Vec<_>>();
    commands.entity(*layer_entity).push_children(&children);

    let bundle_batch: Vec<(Entity, (HexTile, SpriteBundle))> = children
        .into_iter()
        .zip(hexes.iter().map(|x| {
            let pos = LAYOUT.hex_to_world_pos(*x);
            (
                HexTile(*x),
                SpriteBundle {
                    texture: asset.clone(),
                    transform: Transform::from_xyz(
                        pos.x,
                        pos.y,
                        MapLayer::to_layer_level(&layer.layer_type),
                    ),
                    ..default()
                },
            )
        }))
        .collect();
    commands.insert_or_spawn_batch(bundle_batch);
}

fn tile_in_layer_added<T: Component, const LAYER_ID: u32>(
    mut commands: Commands,
    q: Query<&HexTile, (Added<T>, (With<HexTile>, With<BaseHex>))>,
    layer_q: Query<(Entity, &Layer)>,
    asset_server: Res<AssetServer>,
) {
    if q.is_empty() {
        return;
    }
    if let Some(layer_type) = MapLayer::from_id(LAYER_ID) {
        let texture_handle: Handle<Image> = asset_server.load(MapLayer::get_texture(layer_type));
        for (layer_entity, layer) in layer_q.iter() {
            if layer_type == layer.layer_type {
                spawn_tiles(
                    &mut commands,
                    &layer_entity,
                    layer,
                    q.iter().map(|x| x.0).collect::<Vec<_>>(),
                    &texture_handle,
                );
            }
        }
    }
}

fn tile_in_layer_removed<T: Component, const LAYER_ID: u32>(
    mut commands: Commands,
    mut select_removed: RemovedComponents<T>,
    tile_q: Query<&HexTile, With<BaseHex>>,
    layer_q: Query<(&Layer, &Children)>,
    layer_tiles: Query<&HexTile, Without<BaseHex>>,
) {
    for hex_tile in tile_q.iter_many(select_removed.iter()) {
        if let Some(layer_type) = MapLayer::from_id(LAYER_ID) {
            for (layer, children) in layer_q.iter() {
                if layer_type == layer.layer_type {
                    for child in children.iter() {
                        let hex = layer_tiles.get(*child).unwrap();
                        if hex.0 == hex_tile.0 {
                            commands.entity(*child).remove_parent().despawn();
                        }
                    }
                }
            }
        }
    }
}
