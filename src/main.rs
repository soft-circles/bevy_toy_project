use bevy::{
    asset::ChangeWatcher, input::common_conditions::input_toggle_active, prelude::*,
    utils::Duration,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use components::{BoardLoc, HexTile, Layer, Unit};
use controls::cursor::CursorPlugin;
use events::EventsPlugin;
use helpers::unit::UnitPlugin;
use player::PlayerPlugin;
use resources::*;
use startup::StartupPlugin;
use states::{AppState, PlayerState};
use tiles::{layers::LayersPlugin, TilePlugin};
use turn_queue::TurnQueuePlugin;
use ui::GameUI;

mod bundles;
mod components;
mod constants;
mod controls;
mod enums;
mod events;
mod helpers;
mod player;
mod resources;
mod startup;
mod states;
mod tiles;
mod turn_queue;
mod ui;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Lichdom"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                }),
        )
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .init_resource::<CursorPos>()
        .init_resource::<TurnQueue>()
        .init_resource::<HexMap>()
        .add_state::<AppState>()
        .add_state::<PlayerState>()
        .register_type::<Unit>()
        .register_type::<BoardLoc>()
        .register_type::<HexTile>()
        .register_type::<Layer>()
        .add_plugins(EventsPlugin)
        .add_plugins(StartupPlugin)
        .add_plugins(GameUI)
        .add_plugins(TurnQueuePlugin)
        .add_plugins(CursorPlugin)
        .add_plugins(TilePlugin)
        .add_plugins(UnitPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(LayersPlugin)
        .add_systems(Update, helpers::camera::movement)
        .run();
}
