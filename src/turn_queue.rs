use bevy::prelude::*;

use crate::{events::TurnButtonPressed, resources::TurnQueue};

pub struct TurnQueuePlugin;

fn update_turn_number(
    mut ev_turn_button_pressed: EventReader<TurnButtonPressed>,
    mut turn_number: ResMut<TurnQueue>,
) {
    for _ in ev_turn_button_pressed.iter() {
        turn_number.turn_number += 1;
    }
}

impl Plugin for TurnQueuePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_turn_number);
    }
}
