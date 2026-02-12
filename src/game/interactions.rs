use bevy::{prelude::*, window::PrimaryWindow};

use crate::AppSystems;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, interact.in_set(AppSystems::HandleInput));
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Interactible {
    bounds: Bounds,
}

#[derive(Debug, Copy, Clone, Reflect)]
pub struct Bounds {
    pub position: Vec2,
    pub size: Vec2,
}

impl Bounds {
    #[inline]
    pub fn in_bounds(&self, coords: Vec2) -> bool {
        coords.x >= self.position.x
            && coords.y >= self.position.y
            && coords.x <= self.position.x + self.size.x
            && coords.y <= self.position.y + self.size.y
    }
}

#[derive(EntityEvent, Debug)]
struct MouseClick {
    #[event_target]
    entity: Entity,
}

fn interact(
    mut commands: Commands,
    window: Single<&mut Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    interactible_query: Query<(Entity, &Interactible)>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let mouse_position = match window.cursor_position() {
        Some(position) => {
            let window_size = Vec2::new(window.width(), window.height());
            position - window_size / 2.0
        }
        None => return,
    };
    for (entity, interactible) in interactible_query {
        if interactible.bounds.in_bounds(mouse_position) {
            commands.trigger(MouseClick { entity });
        }
    }
}
