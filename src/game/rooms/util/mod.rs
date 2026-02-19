use bevy::prelude::*;

pub mod room_switch;
pub mod transitions;

pub(super) fn plugin(app: &mut App) {
    #[rustfmt::skip]
    app.add_plugins((
        room_switch::plugin,
        transitions::plugin,
    ));
}
