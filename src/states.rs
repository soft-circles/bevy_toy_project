use bevy::prelude::States;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState {
    #[default]
    LoadingMap,
    InGame,
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    UnitSelected,
    UnitMoving,
}
