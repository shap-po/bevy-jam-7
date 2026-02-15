use bevy::{prelude::*, state::commands, window::PrimaryWindow};

use crate::AppSystems;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(SpritePickingSettings {
        require_markers: false,
        picking_mode: SpritePickingMode::AlphaThreshold(0.01),
    });
}
