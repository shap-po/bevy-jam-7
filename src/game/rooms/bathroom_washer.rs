use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_bundled_observers::observers;

use crate::asset_tracking::LoadResource;
use crate::audio::{PlaySfx, Sfxlib};
use crate::game::enemies::washer::{Washer, WashingMinigameComplete};
use crate::game::rooms::util::room_switch::{ArrowDirection, arrow_room_switcher};
use crate::game::rooms::{Background, Room, RoomComponent};
use crate::utils::SetImage;
use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<BathroomWasherAssets>();
    app.add_systems(
        Update,
        (set_background, update_text)
            .run_if(in_state(Room::BathroomWasher))
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
    app.add_systems(OnEnter(Room::BathroomWasher), start_washer_loop);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub(super) struct BathroomWasherAssets {
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
pub(super) fn room(assets: &BathroomWasherAssets) -> impl Bundle {
    (
        RoomComponent {
            this_room: Room::BathroomWasher,
            back_room: Some(Room::Bathroom),
            ..Default::default()
        },
        children![
            (
                WasherSprite,
                Sprite::from_image(assets.washer.clone()),
                Pickable::default(),
                observers![
                    update_washer::<Pointer<Release>>(),
                ],
            ),
            (
                WasherText,
                Anchor(Vec2 { x: 0.5, y: -2.0 }),
                Text2d::new("00:00"),
                TextColor::BLACK,
                TextFont::from_font_size(40.0),
            ),
            arrow_room_switcher(Room::Bathroom, ArrowDirection::Down),
        ],
    )
}

fn set_background(
    mut sprite: Single<&mut Sprite, With<Background>>,
    assets: If<Res<BathroomWasherAssets>>,
) {
    sprite.set_image(&assets.background);
}

fn update_text(mut text: Single<&mut Text2d, With<WasherText>>, washer: Single<&Washer>) {
    text.0 = match **washer {
        Washer::Working(time) => format!("{:>2}:00", time),
        Washer::Waiting(_) => "!!!!".to_string(),
        Washer::Death => "boom".to_string(),
    }
}

fn update_washer<E>() -> impl Fn(On<E>, Query<&Sprite>, Commands, Single<&Washer>)
where
    E: EntityEvent + std::fmt::Debug + Clone + Reflect,
{
    move |ev, sprites, mut commands, washer| {
        let Ok(_) = sprites.get(ev.event_target()) else {
            return;
        };
        let Washer::Waiting(_) = **washer else {
            return;
        };
        commands.trigger(WashingMinigameComplete);
    }
}

fn start_washer_loop(mut commands: Commands, sfx_asset: Res<Sfxlib>, washer: Single<&Washer>) {
    let Washer::Working(_) = **washer else {
        return;
    };
    commands.play_loop_sfx(
        sfx_asset.washer_ambient_loop.clone(),
        0.4,
        Room::BathroomWasher,
    );
}
