pub mod layers;

use bevy::prelude::*;
use hexx::Hex;

use crate::{
    components::{Activated, BaseHex, BoardLoc, HexTile, Hovered, Selectable, Selected, Unit},
    constants::LAYOUT,
    events::{
        ClearLastClicked, HexDoubleClicked, MouseClicked, MouseClickedHex, MouseEnteredHex,
        NewTileClicked, UnitDeselected, UnitSelected,
    },
    resources::{CursorPos, HexMap},
    states::{AppState, PlayerState},
};
#[derive(Default)]
struct LastHexEntered(pub Option<Hex>);

#[derive(Default)]
struct LastTileClicked {
    pub hex: Option<Hex>,
}

#[derive(Default)]
struct LastSelectedUnit(pub Option<Entity>);

fn check_mouse_entered_tile(
    cursor_pos: Res<CursorPos>,
    hex_map: Res<HexMap>,
    mut ev_writer: EventWriter<MouseEnteredHex>,
    mut local: Local<LastHexEntered>,
) {
    let hex = LAYOUT.world_pos_to_hex(cursor_pos.0);
    if !hex_map.0.contains(&hex) {
        return;
    };
    if let Some(last_entered_tile) = local.0 {
        if last_entered_tile != hex {
            local.0 = Some(hex);
            ev_writer.send(MouseEnteredHex(hex));
        }
    } else {
        local.0 = Some(hex);
        ev_writer.send(MouseEnteredHex(hex));
    }
}

#[derive(Default)]
struct LastHoveredTile(pub Option<Hex>);

fn remove_hover_from_tile(
    mut commands: Commands,
    mut ev_reader: EventReader<MouseEnteredHex>,
    mut local: Local<LastHoveredTile>,
    tile_q: Query<(Entity, &HexTile), With<BaseHex>>,
) {
    for ev in ev_reader.iter() {
        if let Some(hex) = local.0 {
            if hex != ev.0 {
                if let Some((entity, _)) = tile_q.iter().find(|&(_, tile)| tile.0 == hex) {
                    local.0 = Some(ev.0);
                    commands.entity(entity).remove::<Hovered>();
                }
            }
        } else {
            local.0 = Some(ev.0);
        }
    }
}

fn add_hovered_to_tile(
    mut commands: Commands,
    mut ev_reader: EventReader<MouseEnteredHex>,
    hex_tiles: Query<(Entity, &HexTile), With<BaseHex>>,
) {
    for ev in ev_reader.iter() {
        for hex_tile in hex_tiles.iter() {
            if hex_tile.1 .0 == ev.0 {
                commands.entity(hex_tile.0).insert(Hovered);
            }
        }
    }
}

fn send_mouse_clicked_hex_event(
    mut ev_mouse_clicked: EventReader<MouseClicked>,
    hex_map: Res<HexMap>,
    mut ev_mouse_clicked_tile_writer: EventWriter<MouseClickedHex>,
) {
    for ev in ev_mouse_clicked.iter() {
        let hex = LAYOUT.world_pos_to_hex(ev.0);
        if let Some(x) = hex_map.0.get(&hex) {
            ev_mouse_clicked_tile_writer.send(MouseClickedHex(*x));
        }
    }
}

fn add_selected_to_tile(
    mut commands: Commands,
    mut ev_mouse_clicked_tile: EventReader<NewTileClicked>,
    selected_tile_q: Query<(Entity, &HexTile), With<Selected>>,
    tiles_q: Query<(Entity, &HexTile)>,
) {
    for ev in ev_mouse_clicked_tile.iter() {
        selected_tile_q.iter().for_each(|x| {
            commands.entity(x.0).remove::<Selected>();
        });
        if let Some(x) = tiles_q.iter().find(|x| x.1 .0 == ev.0) {
            commands.entity(x.0).insert(Selected);
        }
    }
}

fn send_new_tile_clicked_event(
    mut ev_mouse_clicked_hex: EventReader<MouseClickedHex>,
    mut ev_new_tile_clicked: EventWriter<NewTileClicked>,
    mut last_tile_clicked_local: Local<LastTileClicked>,
    mut ev_clear_last_clicked: EventReader<ClearLastClicked>,
) {
    for _ in ev_clear_last_clicked.iter() {
        last_tile_clicked_local.hex = None;
    }
    for ev in ev_mouse_clicked_hex.iter() {
        match last_tile_clicked_local.hex {
            Some(x) => {
                if x != ev.0 {
                    last_tile_clicked_local.hex = Some(ev.0);
                    ev_new_tile_clicked.send(NewTileClicked(ev.0));
                }
            }
            None => {
                last_tile_clicked_local.hex = Some(ev.0);
                ev_new_tile_clicked.send(NewTileClicked(ev.0));
            }
        };
    }
}

fn remove_selected_from_unit(
    mut commands: Commands,
    mut ev_new_tile_clicked: EventReader<NewTileClicked>,
    unit_q: Query<(Entity, &BoardLoc), (With<Selectable>, With<Unit>, With<Selected>)>,
) {
    if let Some((_, (unit_entity, _))) = ev_new_tile_clicked
        .iter()
        .zip(unit_q.iter())
        .find(|&(ev, (_, board_loc))| board_loc.hex != ev.0)
    {
        commands.entity(unit_entity).remove::<Selected>();
    }
}

fn add_selected_to_unit(
    mut commands: Commands,
    mut ev_new_tile_clicked: EventReader<NewTileClicked>,
    unit_q: Query<(Entity, &BoardLoc), (With<Selectable>, With<Unit>, Without<Selected>)>,
) {
    if let Some((_, (unit_entity, _))) = ev_new_tile_clicked
        .iter()
        .zip(unit_q.iter())
        .find(|&(ev, (_, board_loc))| board_loc.hex == ev.0)
    {
        commands.entity(unit_entity).insert(Selected);
    }
}

fn send_unit_selected_event(
    unit_q: Query<Entity, (With<Selectable>, With<Unit>, Added<Selected>)>,
    mut ev_unit_selected: EventWriter<UnitSelected>,
) {
    for unit_entity in unit_q.iter() {
        ev_unit_selected.send(UnitSelected(unit_entity));
    }
}

fn send_unit_deselected_event(
    mut unit_selected_removed: RemovedComponents<Selected>,
    unit_q: Query<Entity, With<Unit>>,
    mut ev_unit_deselected: EventWriter<UnitDeselected>,
) {
    for deselected_entity in unit_selected_removed.iter() {
        if unit_q.contains(deselected_entity) {
            ev_unit_deselected.send(UnitDeselected(deselected_entity));
        }
    }
}

fn send_hex_double_clicked_event(
    mut ev_writer: EventWriter<HexDoubleClicked>,
    mut ev_mouse_clicked_hex: EventReader<MouseClickedHex>,
    mut last_tile_clicked: Local<LastTileClicked>,
) {
    for mouse_clicked_ev in ev_mouse_clicked_hex.iter() {
        match last_tile_clicked.hex {
            Some(x) => {
                if x == mouse_clicked_ev.0 {
                    ev_writer.send(HexDoubleClicked(x));
                }
                last_tile_clicked.hex = Some(mouse_clicked_ev.0);
            }
            None => last_tile_clicked.hex = Some(mouse_clicked_ev.0),
        }
    }
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_mouse_entered_tile,
                send_mouse_clicked_hex_event,
                add_hovered_to_tile,
                remove_hover_from_tile,
                add_selected_to_tile,
                add_selected_to_unit,
                send_new_tile_clicked_event,
                remove_selected_from_unit,
                send_unit_selected_event,
                send_unit_deselected_event,
            )
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(PlayerState::Idle)),
        )
        .add_systems(
            Update,
            (
                send_new_tile_clicked_event,
                check_mouse_entered_tile,
                send_mouse_clicked_hex_event,
                add_hovered_to_tile,
                remove_hover_from_tile,
                add_selected_to_tile,
                send_hex_double_clicked_event,
            )
                .run_if(in_state(PlayerState::UnitSelected)),
        )
        .add_systems(
            Update,
            (
                check_mouse_entered_tile,
                add_hovered_to_tile,
                remove_hover_from_tile,
            )
                .run_if(in_state(PlayerState::UnitMoving)),
        );
    }
}
