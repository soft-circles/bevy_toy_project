use bevy::{math::Vec3Swizzles, prelude::*};
use hexx::{algorithms::field_of_movement, Hex};

use crate::{
    components::{
        Activated, BaseHex, BoardLoc, HexTile, MoveRange, MoveTarget, Moving, Path, Selected, Unit,
    },
    constants::LAYOUT,
    events::{
        ClickedOutsideActivationRange, HexDoubleClicked, MoveTargetConfirmed, NewTileClicked,
    },
    resources::HexMap,
    states::PlayerState,
};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Moving>()
            .register_type::<Path>()
            .add_systems(OnEnter(PlayerState::UnitSelected), add_activated_to_tiles)
            .add_systems(
                Update,
                (
                    add_move_target_to_tile.run_if(did_not_click_selected_unit),
                    send_move_target_confirmed_event,
                    on_move_target_confirmed,
                    on_clicked_outside_activation_range,
                )
                    .run_if(in_state(PlayerState::UnitSelected)),
            )
            .add_systems(
                Update,
                (move_along_path, flip_sprite_when_moving)
                    .run_if(in_state(PlayerState::UnitMoving)),
            );
    }
}

fn on_clicked_outside_activation_range(
    mut ev_new_tile_clicked: EventReader<NewTileClicked>,
    tiles_q: Query<&HexTile, (With<BaseHex>, Without<Activated>)>,
    mut ev_clicked_outside: EventWriter<ClickedOutsideActivationRange>,
) {
    for ev in ev_new_tile_clicked.iter() {
        for tile in tiles_q.iter() {
            if ev.0 == tile.0 {
                ev_clicked_outside.send(ClickedOutsideActivationRange(ev.0));
                return;
            }
        }
    }
}

fn add_activated_to_tiles(
    mut commands: Commands,
    unit_q: Query<(&BoardLoc, &MoveRange), (With<Selected>, With<Unit>)>,
    tile_q: Query<(Entity, &HexTile), With<BaseHex>>,
    hex_map: Res<HexMap>,
) {
    if let Ok((board_loc, move_range)) = unit_q.get_single() {
        let mut result = field_of_movement(board_loc.hex, move_range.0, |_h| Some(0));
        result.insert(board_loc.hex);
        for hex_result in result.iter() {
            if let Some(tile_pos) = hex_map.0.get(hex_result) {
                for (tile_entity, hex_tile) in tile_q.iter() {
                    if hex_tile.0 == *tile_pos {
                        commands.entity(tile_entity).insert(Activated);
                    }
                }
            }
        }
    }
}

fn add_move_target_to_tile(
    mut commands: Commands,
    mut ev_new_tile_clicked: EventReader<NewTileClicked>,
    tile_q: Query<(Entity, &HexTile), (With<BaseHex>, With<Activated>, Without<Selected>)>,
    move_target_q: Query<Entity, (With<BaseHex>, With<Activated>, With<MoveTarget>)>,
) {
    for ev in ev_new_tile_clicked.iter() {
        if let Some((entity, _)) = tile_q.iter().find(|(_, hex_tile)| hex_tile.0 == ev.0) {
            for existing_move_target in move_target_q.iter() {
                commands.entity(existing_move_target).remove::<MoveTarget>();
            }
            commands
                .entity(entity)
                .insert(MoveTarget(ev.0))
                .insert(Selected);
        }
    }
}

fn send_move_target_confirmed_event(
    mut ev_double_clicked: EventReader<HexDoubleClicked>,
    tile_q: Query<&HexTile, (With<BaseHex>, With<MoveTarget>)>,
    unit_q: Query<(Entity, &BoardLoc), (With<Selected>, With<Unit>)>,
    mut move_target_ev: EventWriter<MoveTargetConfirmed>,
) {
    for ev in ev_double_clicked.iter() {
        if let Ok((unit_entity, board_loc)) = unit_q.get_single() {
            if let Some(move_target) = tile_q
                .iter()
                .find(|&x| x.0 == ev.0 && ev.0 != board_loc.hex)
            {
                move_target_ev.send(MoveTargetConfirmed {
                    unit: unit_entity,
                    from: board_loc.hex,
                    to: move_target.0,
                });
            }
        }
    }
}

fn on_move_target_confirmed(
    mut commands: Commands,
    mut move_target_ev: EventReader<MoveTargetConfirmed>,
    mut unit_q: Query<(Entity, &mut Transform, &mut BoardLoc), (With<Unit>, With<Selected>)>,
) {
    for ev in move_target_ev.iter() {
        if let Some((unit_entity, _unit_transform, _board_loc)) =
            unit_q.iter_mut().find(|(entity, _, _)| *entity == ev.unit)
        {
            if let Some(path) = hexx::algorithms::a_star(ev.from, ev.to, |_| Some(0)) {
                let hexes: Vec<Hex> = path.iter().take(2).map(|x| *x).collect();
                let direction = hexes
                    .get(0)
                    .unwrap()
                    .neighbor_direction(*hexes.get(1).unwrap())
                    .unwrap();
                commands
                    .entity(unit_entity)
                    .insert(Path(path.iter().skip(1).map(|x| *x).collect::<Vec<Hex>>()))
                    .insert(Moving {
                        direction,
                        towards: path.into_iter().take(2).into_iter().last().unwrap(),
                    });
            }
        }
    }
}

fn flip_sprite_when_moving(mut q: Query<(&mut Sprite, &Moving), Changed<Moving>>) {
    for (mut sprite, moving) in q.iter_mut() {
        match moving.direction {
            hexx::Direction::TopRight => sprite.flip_x = false,
            hexx::Direction::Top => (),
            hexx::Direction::TopLeft => sprite.flip_x = true,
            hexx::Direction::BottomLeft => sprite.flip_x = true,
            hexx::Direction::Bottom => (),
            hexx::Direction::BottomRight => sprite.flip_x = false,
        }
    }
}

fn move_along_path(
    mut commands: Commands,
    mut transform_q: Query<(
        Entity,
        &mut Transform,
        &mut Moving,
        &mut Path,
        &mut BoardLoc,
    )>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut moving, mut path, mut board_loc) in transform_q.iter_mut() {
        let Vec2 { x, y } = LAYOUT.hex_to_world_pos(moving.towards);

        transform.translation = transform
            .translation
            .lerp(Vec3 { x, y, z: 10.0 }, 5.0 * time.delta_seconds());
        if transform.translation.xy().round() == Vec2::new(x, y).round() {
            if let Some(hex) = path.0.clone().iter().next() {
                let next_direction = board_loc.hex.neighbor_direction(*hex).unwrap();
                board_loc.set_if_neq(BoardLoc { hex: *hex });
                moving.set_if_neq(Moving {
                    towards: *hex,
                    direction: next_direction,
                });
                path.set_if_neq(Path(
                    path.0.clone().into_iter().skip(1).collect::<Vec<Hex>>(),
                ));
            } else {
                *transform = Transform::from_xyz(x, y, 10.0);
                commands.entity(entity).remove::<Path>().remove::<Moving>();
            }
        }
    }
}

fn did_not_click_selected_unit(
    mut ev_new_tile_clicked: EventReader<NewTileClicked>,
    unit_q: Query<&BoardLoc, (With<Selected>, With<Unit>)>,
) -> bool {
    for ev in ev_new_tile_clicked.iter() {
        return unit_q
            .iter()
            .find(|&board_loc| board_loc.hex != ev.0)
            .is_some();
    }
    false
}
