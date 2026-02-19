use bevy::prelude::*;
use bevy_bundled_observers::observers;

use crate::game::rooms::{Room, util::transitions::ChangeRoom};

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct RoomSwitcher {
    room: Room,
    pub active: bool,
}

#[rustfmt::skip]
pub fn room_switcher(room:Room, image: Handle<Image>) -> impl Bundle{
    let mut sprite = Sprite::from_image(image);
    sprite.color.set_alpha(0.0); // for now, no need to render; use sprite for picking

    (
        RoomSwitcher{
            room,
            active: true,
        },
        sprite,
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
    if !room_switch.active {
        return;
    }

    commands.trigger(ChangeRoom(room_switch.room));
}
