use bevy::prelude::*;
use std::time::Duration;

use crate::{
    AppSystems, PausableSystems,
    audio::{PlaySfx, Sfxlib},
    game::rooms::Room,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<RoomTransition>();

    app.add_systems(OnExit(Screen::Gameplay), reset_transition);
    app.add_systems(
        Update,
        (handle_transition)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_observer(handle_room_change);
    app.add_observer(handle_transition_sfx);
}

const ROOM_TRANSITION_DURATION: Duration = Duration::from_millis(800);

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct RoomTransition {
    next_room: Option<Room>,
    timer: Timer,
}

impl Default for RoomTransition {
    fn default() -> Self {
        Self {
            next_room: None,
            timer: Timer::new(ROOM_TRANSITION_DURATION, TimerMode::Once),
        }
    }
}

/// Trigger this event to change rooms
#[derive(Event, Debug)]
pub struct ChangeRoom(pub Room);

fn handle_room_change(
    event: On<ChangeRoom>,
    current_room: Res<State<Room>>,
    mut next_room: ResMut<NextState<Room>>,
    mut room_transition: ResMut<RoomTransition>,
    mut commands: Commands,
) {
    let current_room = **current_room;

    next_room.set(Room::Transition);
    room_transition.next_room = Some(event.0);
    room_transition.timer.reset();
    room_transition.timer.unpause();

    commands.trigger(RoomTransitionEvent {
        from: current_room,
        to: event.0,
    });
}

#[derive(Event, Debug)]
struct RoomTransitionEvent {
    from: Room,
    to: Room,
}

fn reset_transition(
    mut next_room: ResMut<NextState<Room>>,
    mut room_transition: ResMut<RoomTransition>,
) {
    next_room.set(Room::Bedroom);
    room_transition.next_room = None;
    room_transition.timer.reset();
}

fn handle_transition(
    mut next_room: ResMut<NextState<Room>>,
    mut room_transition: ResMut<RoomTransition>,
    time: Res<Time>,
) {
    let Some(room) = room_transition.next_room else {
        return;
    };

    room_transition.timer.tick(time.delta());
    if !room_transition.timer.is_finished() {
        return;
    }

    next_room.set(room);
    room_transition.next_room = None;
}

fn handle_transition_sfx(
    event: On<RoomTransitionEvent>,
    sfx_asset: Res<Sfxlib>,
    mut commands: Commands,
) {
    if (event.to == Room::Bedroom && event.from != Room::BedroomBed)
        || (event.from == Room::Bedroom && event.to != Room::BedroomBed)
    {
        commands.play_volume_sfx(sfx_asset.rand_door_open_and_close(), 0.05);
    }
    if event.to == Room::KitchenFridge && event.from == Room::Kitchen {
        //Kitchen -> Fridge
        commands.play_volume_sfx(sfx_asset.rand_fridge_open(), 0.5);
    }
    if event.to == Room::Kitchen && event.from == Room::KitchenFridge {
        //Fridge -> Kitchen
        commands.play_volume_sfx(sfx_asset.rand_fridge_close(), 0.5);
    }
    if (event.from != Room::BedroomBed) && (event.to != Room::BedroomBed) {
        commands.play_simple_sfx(sfx_asset.rand_player_run());
    } else {
        commands.play_simple_sfx(sfx_asset.rand_blanket());
    }
}
