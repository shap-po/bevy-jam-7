use bevy::prelude::*;
use bevy_bundled_observers::observers;

use crate::game::rooms::{Room, util::transitions::ChangeRoom};

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct RoomSwitcher(Room);

#[rustfmt::skip]
pub fn room_switcher(room:Room, image: Handle<Image>) -> impl Bundle{
    (
        RoomSwitcher(room),
        Sprite::from_image(image),
        Pickable::default(),
        observers![
            handle_press,
        ],
    )
}

fn handle_press(
    ev: On<Pointer<Release>>,
    room_switch_query: Query<&RoomSwitcher>,
    mut commands: Commands,
) {
    let Ok(room_switch) = room_switch_query.get(ev.event_target()) else {
        return;
    };

    commands.trigger(ChangeRoom(room_switch.0));
}
