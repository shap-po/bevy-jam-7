use bevy::prelude::*;

pub mod room_switch;
pub mod transitions;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(transitions::plugin);
}
