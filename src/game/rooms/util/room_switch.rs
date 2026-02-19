use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_bundled_observers::observers;

use crate::{
    WINDOW_HEIGHT, WINDOW_WIDTH,
    game::rooms::{Room, util::transitions::ChangeRoom},
    utils::DeferredSprite,
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<RoomSwitchAssets>();
    app.add_systems(Update, handle_active_change);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct RoomSwitcher {
    room: Room,
    pub active: bool,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ArrowRoomSwitcher;

const ARROW_MARGIN: f32 = 80.0;

pub enum ArrowDirection {
    Up,
    Down,
    Left,
    Right,
}

impl ArrowDirection {
    fn get_transform(&self) -> Transform {
        match self {
            ArrowDirection::Up => Transform::from_translation(Vec3 {
                x: 0.0,
                y: WINDOW_HEIGHT / 2.0 - ARROW_MARGIN,
                z: 1.0,
            }),
            ArrowDirection::Down => Transform::from_translation(Vec3 {
                x: 0.0,
                y: -WINDOW_HEIGHT / 2.0 + ARROW_MARGIN,
                z: 1.0,
            })
            .with_rotation(Quat::from_rotation_z(PI)),
            ArrowDirection::Left => Transform::from_translation(Vec3 {
                x: -WINDOW_WIDTH / 2.0 + ARROW_MARGIN,
                y: 0.0,
                z: 1.0,
            })
            .with_rotation(Quat::from_rotation_z(PI / 2.0)),
            ArrowDirection::Right => Transform::from_translation(Vec3 {
                x: WINDOW_WIDTH / 2.0 - ARROW_MARGIN,
                y: 0.0,
                z: 1.0,
            })
            .with_rotation(Quat::from_rotation_z(PI * 3.0 / 2.0)),
        }
    }
}

#[rustfmt::skip]
pub fn room_switcher(room: Room, image: Handle<Image>) -> impl Bundle{
    let mut sprite = Sprite::from_image(image);
    sprite.color.set_alpha(0.0); // for now, no need to render; use sprite for picking

    (
        ArrowRoomSwitcher,
        RoomSwitcher {
            room,
            active: true,
        },
        sprite,
        Pickable::default(),
        Visibility::default(),
        observers![
            handle_press,
        ],
    )
}

#[rustfmt::skip]
pub fn arrow_room_switcher(room: Room, direction: ArrowDirection) -> impl Bundle {
    (
        RoomSwitcher {
            room,
            active: true,
        },
        DeferredSprite::new(|assets: &RoomSwitchAssets| assets.arrow.clone()),
        Pickable::default(),
        Visibility::default(),
        direction.get_transform(),
        observers![
            handle_press,
        ],
    )
}

fn handle_active_change(query: Query<(&mut Visibility, &RoomSwitcher), Changed<RoomSwitcher>>) {
    for (mut visibility, room_switch) in query {
        *visibility = if room_switch.active {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

fn handle_press(
    ev: On<Pointer<Release>>,
    room_switch_query: Query<&RoomSwitcher>,
    mut commands: Commands,
) {
    let Ok(room_switch) = room_switch_query.get(ev.event_target()) else {
        return;
    };
    if !room_switch.active {
        return;
    }

    commands.trigger(ChangeRoom(room_switch.room));
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
#[rustfmt::skip]
pub(super) struct RoomSwitchAssets {
    #[dependency] arrow: Handle<Image>,
}

impl FromWorld for RoomSwitchAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            arrow: assets.load("images/rooms/prop/arrow.png"),
        }
    }
}
