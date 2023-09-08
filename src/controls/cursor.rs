use bevy::prelude::*;

use crate::{events::MouseClicked, resources::CursorPos, states::AppState};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_cursor_pos, cursor_clicked).run_if(in_state(AppState::InGame)),
        );
    }
}

impl Default for CursorPos {
    fn default() -> Self {
        // Initialize the cursor pos at some far away place. It will get updated
        // correctly when the cursor moves.
        Self(Vec2::new(-1000.0, -1000.0))
    }
}

// We need to keep the cursor position updated based on any `CursorMoved` events.
pub fn update_cursor_pos(
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    for cursor_moved in cursor_moved_events.iter() {
        for (cam_t, cam) in camera_q.iter() {
            if let Some(pos) = cam.viewport_to_world_2d(cam_t, cursor_moved.position) {
                *cursor_pos = CursorPos(pos);
            }
        }
    }
}

pub fn cursor_clicked(
    btn: Res<Input<MouseButton>>,
    cursor_pos: Res<CursorPos>,
    mut ev_mouse_clicked: EventWriter<MouseClicked>,
) {
    if !btn.just_pressed(MouseButton::Left) {
        return;
    }

    ev_mouse_clicked.send(MouseClicked(cursor_pos.0));
}
