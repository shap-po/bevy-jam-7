use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::state::commands;
use bevy_bundled_observers::observers;
use leafwing_input_manager::prelude::ActionState;

use crate::asset_tracking::LoadResource;
use crate::game::enemies::washer::{Washer, WashingMinigameComplete};
use crate::game::rooms::{Background, Room, RoomComponent};
use crate::input_manager::Action;
use crate::screens::Screen;
use crate::theme::widget;
use crate::utils::SetImage;
use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<BathroomWasherAssets>();
    app.add_systems(
        Update,
        (set_background, set_washer, update_text).run_if(in_state(Room::BathroomWasher)),
    );
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct BathroomWasherAssets {
    #[dependency]
    background: Handle<Image>,
    #[dependency]
    washer: Handle<Image>,
}

impl FromWorld for BathroomWasherAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load("images/rooms/bathroom_washer.png"),
            washer: assets.load("images/rooms/prop/washer.png"),
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct WasherSprite;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct WasherText;

#[rustfmt::skip]
pub(super) fn room() -> impl Bundle {
    (
        RoomComponent {
            this_room: Room::BathroomWasher,
            back_room: Some(Room::Bathroom),
            ..Default::default()
        },
        children![
            (
                WasherSprite,
                Sprite::default(),
                Pickable::default(),
                observers![
                    update_washer::<Pointer<Press>>(),
                ],
            ),
            (
                WasherText,
                Anchor(Vec2 { x: 0.5, y: -2.0 }),
                Text2d::new("00:00"),
                TextColor::BLACK,
                TextFont::from_font_size(40.0),
            ),
        ],
    )
}

fn set_background(
    mut sprite: Single<&mut Sprite, With<Background>>,
    assets: If<Res<BathroomWasherAssets>>,
) {
    sprite.set_image(&assets.background);
}

fn set_washer(
    mut sprite: Single<&mut Sprite, With<WasherSprite>>,
    assets: If<Res<BathroomWasherAssets>>,
) {
    sprite.set_image(&assets.washer);
}

fn update_text(mut text: Single<&mut Text2d, With<WasherText>>, washer: Single<&Washer>) {
    text.0 = match **washer {
        Washer::Passive(time) => format!("{:>2}:00", time),
        Washer::Active(_) => "!!!!".to_string(),
        Washer::Death => "boom".to_string(),
    }
}

fn update_washer<E>() -> impl Fn(On<E>, Query<&Sprite>, Commands, Single<&Washer>)
where
    E: EntityEvent + std::fmt::Debug + Clone + Reflect,
{
    move |ev, sprites, mut commands, washer| {
        let Ok(sprite) = sprites.get(ev.event_target()) else {
            return;
        };
        let Washer::Active(_) = **washer else {
            return;
        };
        commands.trigger(WashingMinigameComplete);
    }
}
