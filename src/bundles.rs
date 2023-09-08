use bevy::prelude::*;

use crate::components::Layer;

#[derive(Bundle)]
pub struct LayerBundle {
    pub layer: Layer,
    pub global_transform: GlobalTransform,
    pub transform: Transform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}
