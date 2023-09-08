use bevy::prelude::*;

use crate::{
    components::{Activated, BaseHex, MoveTarget, Moving, Selected, Unit},
    events::{ClearLastClicked, ClickedOutsideActivationRange, UnitDeselected, UnitSelected},
    states::PlayerState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                transition_to_select_unit_state,
                transition_to_idle_state,
                transition_to_unit_moving_state,
                on_unit_stop_moving,
            ),
        )
        .add_systems(
            OnExit(PlayerState::UnitSelected),
            (deactivate_units_and_tiles, clear_last_clicked),
        );
    }
}

fn transition_to_select_unit_state(
    mut next_state: ResMut<NextState<PlayerState>>,
    mut ev_unit_selected: EventReader<UnitSelected>,
) {
    for _ in ev_unit_selected.iter() {
        next_state.set(PlayerState::UnitSelected);
    }
}

fn transition_to_idle_state(
    mut next_state: ResMut<NextState<PlayerState>>,
    unit_q: Query<Entity, (With<Unit>, With<Selected>)>,
    mut ev_unit_deselected: EventReader<UnitDeselected>,
    ev_clicked_outside_activation_range: EventReader<ClickedOutsideActivationRange>,
) {
    if !ev_clicked_outside_activation_range.is_empty() {
        next_state.set(PlayerState::Idle);
        return;
    }
    for _ in ev_unit_deselected.iter() {
        if unit_q.is_empty() {
            next_state.set(PlayerState::Idle);
        }
    }
}

fn transition_to_unit_moving_state(
    mut next_state: ResMut<NextState<PlayerState>>,
    unit_q: Query<&Moving>,
) {
    if !unit_q.is_empty() {
        next_state.set(PlayerState::UnitMoving);
    }
}

fn on_unit_stop_moving(
    mut next_state: ResMut<NextState<PlayerState>>,
    stopped_moving: RemovedComponents<Moving>,
) {
    if !stopped_moving.is_empty() {
        next_state.set(PlayerState::Idle)
    }
}

fn clear_last_clicked(mut ev_clear_last_clicked: EventWriter<ClearLastClicked>) {
    ev_clear_last_clicked.send(ClearLastClicked);
}

fn deactivate_units_and_tiles(
    mut commands: Commands,
    unit_q: Query<Entity, (With<Unit>, With<Activated>, With<Selected>)>,
    tile_q: Query<Entity, With<BaseHex>>,
) {
    for unit in unit_q.iter() {
        commands
            .entity(unit)
            .remove::<Selected>()
            .remove::<Activated>();
    }
    for tile in tile_q.iter() {
        commands
            .entity(tile)
            .remove::<Selected>()
            .remove::<Activated>()
            .remove::<MoveTarget>();
    }
}
