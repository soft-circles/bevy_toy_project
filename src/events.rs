use bevy::prelude::*;
use hexx::{Direction, Hex};

#[derive(Event)]
pub struct MapLoaded;

#[derive(Event)]
pub struct TurnButtonPressed;

#[derive(Event)]
pub struct MouseClicked(pub Vec2);

#[derive(Event)]
pub struct MouseEnteredHex(pub Hex);

#[derive(Event)]
pub struct MouseClickedHex(pub Hex);

#[derive(Event)]
pub struct NewTileClicked(pub Hex);

#[derive(Event)]
pub struct UnitSelected(pub Entity);

#[derive(Event)]
pub struct UnitDeselected(pub Entity);

#[derive(Event)]
pub struct HexDoubleClicked(pub Hex);

#[derive(Event)]
pub struct MoveTargetConfirmed {
    pub unit: Entity,
    pub from: Hex,
    pub to: Hex,
}

#[derive(Event)]
pub struct ClearLastClicked;

#[derive(Event)]
pub struct ClickedOutsideActivationRange(pub Hex);

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TurnButtonPressed>()
            .add_event::<MapLoaded>()
            .add_event::<MouseClicked>()
            .add_event::<MouseClickedHex>()
            .add_event::<NewTileClicked>()
            .add_event::<UnitSelected>()
            .add_event::<UnitDeselected>()
            .add_event::<MoveTargetConfirmed>()
            .add_event::<HexDoubleClicked>()
            .add_event::<ClearLastClicked>()
            .add_event::<ClickedOutsideActivationRange>()
            .add_event::<MouseEnteredHex>();
    }
}
