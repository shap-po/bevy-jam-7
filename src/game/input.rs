use bevy::prelude::*;

use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MappedInput>();
    app.add_systems(Update, map_input.in_set(AppSystems::RecordInput));
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct MappedInput {
    pub direction: Vec2,
}

fn map_input(input: Res<ButtonInput<KeyCode>>, mut mapped_input: ResMut<MappedInput>) {
    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    let direction = direction.normalize_or_zero();

    mapped_input.direction = direction;
}
