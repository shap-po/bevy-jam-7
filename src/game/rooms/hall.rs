use bevy::prelude::*;
use bevy_bundled_observers::observers;

use crate::asset_tracking::LoadResource;
use crate::audio::{PlaySfx, Sfxlib};
use crate::game::enemies::doordash::{Doordash, FoodBrought};
use crate::game::rooms::kitchen_fridge_food::FoodAssets;
use crate::game::rooms::{Background, Room, RoomComponent};
use crate::utils::SetImage;
use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<HallAssets>();

    app.add_systems(
        Update,
        (set_background, set_thinking)
            .run_if(in_state(Room::Hall))
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
    app.add_systems(OnEnter(Room::Hall), start_clock_loop);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct HallAssets {
    #[dependency]
    background: Handle<Image>,
    #[dependency]
    background_w_hand: Handle<Image>,
    #[dependency]
    door: Handle<Image>,

    #[dependency]
    thinking: Handle<Image>,
}

impl FromWorld for HallAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load("images/rooms/hall_0.png"),
            background_w_hand: assets.load("images/rooms/hall_1.png"),
            door: assets.load("images/rooms/prop/door.png"),
            thinking: assets.load("images/rooms/prop/thinking.png"),
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Door;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Thinking;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ThinkingWhat;

#[rustfmt::skip]
pub(super) fn room(assets:&HallAssets) -> impl Bundle {
    let mut sprite = Sprite::from_image(assets.door.clone());
    sprite.color.set_alpha(0.0);

    (
        RoomComponent {
            this_room: Room::Hall,
            back_room: Some(Room::Bedroom),
            ..Default::default()
        },
        children![
            (
                Door,
                Pickable::default(),
                sprite,
                observers![
                    give_food,
                ],
            ),
            (
                Thinking,
                Transform::from_xyz(0.0, 0.0, 1.0),
                Sprite::from_image(assets.thinking.clone()),
                Visibility::Hidden,
                children![
                    (
                        ThinkingWhat,
                        Transform::from_xyz(0.0, 0.0, 2.0),
                        Sprite::default(),
                    ),
                ],
            ),
        ],
    )
}

fn set_background(
    mut sprite: Single<&mut Sprite, With<Background>>,
    doordash: Single<&Doordash>,
    assets: If<Res<HallAssets>>,
) {
    sprite.set_image(
        if let Doordash::Waiting {
            state_counter: _,
            food_want: _,
        } = *doordash
        {
            &assets.background_w_hand
        } else {
            &assets.background
        },
    );
}

fn give_food(_: On<Pointer<Press>>, mut commands: Commands) {
    commands.trigger(FoodBrought);
}

fn set_thinking(
    mut thinking: Single<&mut Visibility, With<Thinking>>,
    mut thinking_what: Single<&mut Sprite, With<ThinkingWhat>>,
    doordash: Single<&Doordash>,
    assets: Res<FoodAssets>,
) {
    if let Doordash::Waiting {
        state_counter: _,
        food_want,
    } = *doordash
    {
        **thinking = Visibility::Inherited;
        thinking_what.set_image(&assets.get_thought(*food_want));
    } else {
        **thinking = Visibility::Hidden;
    }
}

fn start_clock_loop(mut commands: Commands, sfx_asset: Res<Sfxlib>) {
    commands.play_loop_sfx(sfx_asset.hall_ambient_clock.clone(), 0.25, Room::Hall);
}
