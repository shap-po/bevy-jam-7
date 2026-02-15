use bevy::prelude::*;
use bevy::state::commands;
use bevy::window::WindowClosed;
use bevy_bundled_observers::observers;
use leafwing_input_manager::prelude::ActionState;

use crate::asset_tracking::LoadResource;
use crate::game::enemies::dino::{Dino, KitchenWindowClosed};
use crate::game::rooms::{Background, Room, RoomComponent};
use crate::input_manager::Action;
use crate::screens::Screen;
use crate::theme::widget;
use crate::utils::SetImage;
use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<KitchenWindowAssets>();
    app.add_systems(
        Update,
        (set_background, set_window, force_move).run_if(in_state(Room::KitchenWindow)),
    );
    app.add_systems(OnExit(Room::KitchenWindow), close_window);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct KitchenWindowAssets {
    #[dependency]
    dino_gone: Handle<Image>,
    #[dependency]
    dino_far: Handle<Image>,
    #[dependency]
    dino_near: Handle<Image>,
    #[dependency]
    dino_stalk: Handle<Image>,
    #[dependency]
    closed_window: Handle<Image>,

    #[dependency]
    window: Handle<Image>,
}

impl FromWorld for KitchenWindowAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            dino_gone: assets.load("images/rooms/kitchen_dino_gone.png"),
            dino_far: assets.load("images/rooms/kitchen_dino_far.png"),
            dino_near: assets.load("images/rooms/kitchen_dino_near.png"),
            dino_stalk: assets.load("images/rooms/kitchen_dino_stalk.png"),
            closed_window: assets.load("images/rooms/kitchen_closed_window.png"),

            window: assets.load("images/rooms/prop/kitchen_window.png"),
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct KitchenWindowRoom;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct KitchenWindow;

#[rustfmt::skip]
pub(super) fn room() -> impl Bundle {
    (
        KitchenWindowRoom,
        RoomComponent {
            this_room: Room::KitchenWindow,
            back_room: Some(Room::Kitchen),
            left_room: Some(Room::KitchenFridge),
            ..Default::default()
        },
        children![
            (
                KitchenWindow,
                Sprite::default(),
                Pickable::default(),
                observers![
                    update_window::<Pointer<Press>>(true),
                    update_window::<Pointer<Release>>(false),
                    update_window::<Pointer<Out>>(false),
                ],
            ),
        ],
    )
}

fn set_background(
    mut sprite: Single<&mut Sprite, With<Background>>,
    dino: Single<&Dino>,
    window: Res<KitchenWindowClosed>,
    assets: If<Res<KitchenWindowAssets>>,
) {
    if window.0 {
        sprite.set_image(&assets.closed_window);
        return;
    }

    sprite.set_image(match **dino {
        Dino::Gone => &assets.dino_gone,
        Dino::Far => &assets.dino_far,
        Dino::Near => &assets.dino_near,
        Dino::Stalk => &assets.dino_stalk,
        Dino::Death => &assets.dino_gone,
    });
}

fn set_window(
    mut sprite: Single<&mut Sprite, With<KitchenWindow>>,
    window: Res<KitchenWindowClosed>,
    assets: If<Res<KitchenWindowAssets>>,
) {
    sprite.set_image(&assets.window);
    sprite.color.set_alpha(if window.0 { 0.0 } else { 1.0 });
}

fn update_window<E>(set_closed: bool) -> impl Fn(On<E>, Query<&Sprite>, ResMut<KitchenWindowClosed>)
where
    E: EntityEvent + std::fmt::Debug + Clone + Reflect,
{
    move |ev, sprites, mut window| {
        let Ok(sprite) = sprites.get(ev.event_target()) else {
            return;
        };
        window.0 = set_closed;
    }
}

fn close_window(mut window: ResMut<KitchenWindowClosed>) {
    window.0 = false;
}

fn force_move(dino: Single<&Dino>, mut next_room: ResMut<NextState<Room>>) {
    if **dino == Dino::Death {
        next_room.set(Room::Kitchen);
    }
}
