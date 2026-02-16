use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    game::enemies::ghost::{Ghost, MAX_STAGE},
    screens::Screen,
    utils::SetImage,
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<GhostAssets>();

    app.add_systems(
        Update,
        update_overlay
            .run_if(in_state(Screen::Gameplay))
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

const TWEEN_COLDER_DELTA: f32 = 0.0001;
const TWEEN_HOTTER_DELTA: f32 = 0.001;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct GhostAssets {
    #[dependency]
    overlay: Handle<Image>,
}

impl FromWorld for GhostAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            overlay: assets.load("images/cold.png"),
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct GhostOverlay;

#[rustfmt::skip]
pub(super) fn overlay() -> impl Bundle {
    let mut sprite = Sprite::default();
    sprite.color.set_alpha(0.0);

    (
        GhostOverlay,
        Transform::from_xyz(0.0, 0.0, 10.0),
        sprite,
    )
}

fn update_overlay(
    ghost: Single<&Ghost>,
    mut overlay: Single<&mut Sprite, With<GhostOverlay>>,
    assets: If<Res<GhostAssets>>,
) {
    overlay.set_image(&assets.overlay);

    let ghost_progress = i8::from(**ghost) as f32 / MAX_STAGE as f32;

    let current_alpha = overlay.color.alpha();
    let mut new_alpha = (ghost_progress - 0.2) / 8.0;
    new_alpha = f32::max(new_alpha, 0.0);

    // tween
    if new_alpha < current_alpha {
        new_alpha = f32::max(current_alpha - TWEEN_HOTTER_DELTA, new_alpha);
    } else if new_alpha > current_alpha {
        new_alpha = f32::min(current_alpha + TWEEN_COLDER_DELTA, new_alpha);
    } else {
        return;
    }
    overlay.color.set_alpha(new_alpha);
}
