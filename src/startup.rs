use crate::{
    bundles::LayerBundle,
    components::{BaseHex, BoardLoc, HexTile, Layer, MoveRange, Selectable},
    constants::{CENTER_HEX, LAYOUT},
    enums::MapLayer,
    resources::HexMap,
    AppState, Unit,
};
use bevy::prelude::*;
use hexx::{shapes, Hex};

pub struct StartupPlugin;

fn generate_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut hex_map: ResMut<HexMap>,
) {
    let texture_handle: Handle<Image> = asset_server.load(MapLayer::get_texture(MapLayer::BASE));
    let entities: Vec<Entity> = shapes::hexagon(CENTER_HEX, 5)
        .map(|hex| {
            hex_map.0.insert(hex);

            let pos = LAYOUT.hex_to_world_pos(hex);
            commands
                .spawn((
                    Name::new(format!("{} {}", hex.x, hex.y)),
                    BaseHex,
                    HexTile(hex),
                    SpriteBundle {
                        texture: texture_handle.clone(),
                        transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                        ..default()
                    },
                ))
                .with_children(|b| {
                    b.spawn(Text2dBundle {
                        text: Text::from_section(
                            format!("{},{}", hex.x, hex.y),
                            TextStyle {
                                font_size: 10.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        transform: Transform::from_xyz(0.0, 0.0, 10.0),
                        ..default()
                    });
                })
                .id()
        })
        .collect();

    let parent_layer = commands
        .spawn((
            Name::new(String::from("BaseLayer")),
            LayerBundle {
                layer: Layer {
                    layer_type: MapLayer::BASE,
                },
                global_transform: GlobalTransform::from_xyz(0.0, 0.0, 0.0),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                visibility: Visibility::Visible,
                computed_visibility: ComputedVisibility::default(),
            },
        ))
        .id();
    commands.entity(parent_layer).push_children(&entities);
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_layers(mut commands: Commands) {
    for layer in MapLayer::iterator().skip(1) {
        commands.spawn((
            Name::new(String::from(layer.to_string())),
            LayerBundle {
                layer: Layer { layer_type: *layer },
                global_transform: GlobalTransform::from_xyz(0.0, 0.0, 0.0),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                visibility: Visibility::Visible,
                computed_visibility: ComputedVisibility::default(),
            },
        ));
    }
}

fn place_starting_unit(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let axial_pos = Hex { x: 1, y: 0 };
    let pos = LAYOUT.hex_to_world_pos(axial_pos);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { ..default() },
            texture: asset_server.load("tidehunter.png"),
            transform: Transform::from_xyz(pos.x, pos.y, 10.0),
            ..default()
        },
        Unit { health: 10 },
        MoveRange(4),
        Selectable,
        BoardLoc { hex: axial_pos },
        Name::new(String::from("Tidehunter")),
    ));

    next_state.set(AppState::InGame)
}

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (startup, generate_grid, spawn_layers))
            .add_systems(Startup, place_starting_unit);
    }
}
