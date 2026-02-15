use std::time::Duration;

use bevy::{prelude::*, render::view::visibility};
use leafwing_input_manager::prelude::ActionState;

use crate::{AppSystems, PausableSystems, input_manager::Action, screens::Screen, theme::widget};

mod bathroom;
mod bathroom_washer;
mod bedroom;
mod bedroom_bed;
mod hall;
mod kitchen;
mod kitchen_fridge;
mod kitchen_window;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Room>();

    app.add_plugins((
        bathroom::plugin,
        bathroom_washer::plugin,
        bedroom::plugin,
        bedroom_bed::plugin,
        hall::plugin,
        kitchen::plugin,
        kitchen_fridge::plugin,
        kitchen_window::plugin,
    ));

    app.add_systems(Startup, startup);
    app.add_systems(Update, show_background);

    app.add_systems(
        Update,
        navigate
            .in_set(AppSystems::HandleInput)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(
        Update,
        show_room
            .in_set(AppSystems::Update)
            .run_if(in_state(Screen::Gameplay)),
    );

    #[cfg(debug_assertions)]
    app.add_systems(PostStartup, room_check);
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
pub enum Room {
    #[default]
    Bedroom,
    BedroomBed,

    Bathroom,
    BathroomWasher,

    Hall,

    Kitchen,
    KitchenWindow,
    KitchenFridge,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Visibility::Hidden)]
struct RoomComponent {
    this_room: Room,
    // WASD navigation
    forward_room: Option<Room>,
    back_room: Option<Room>,
    left_room: Option<Room>,
    right_room: Option<Room>,

    can_move_forward: bool,
    can_move_back: bool,
    can_move_left: bool,
    can_move_right: bool,
}

impl Default for RoomComponent {
    fn default() -> Self {
        Self {
            this_room: Room::Bedroom,

            forward_room: None,
            back_room: None,
            left_room: None,
            right_room: None,

            can_move_forward: true,
            can_move_back: true,
            can_move_left: true,
            can_move_right: true,
        }
    }
}

impl RoomComponent {
    fn is(&self, room: Room) -> bool {
        self.this_room == room
    }

    fn unblock_movement(&mut self) {
        self.can_move_forward = true;
        self.can_move_back = true;
        self.can_move_left = true;
        self.can_move_right = true;
    }

    fn block_movement(&mut self) {
        self.can_move_forward = false;
        self.can_move_back = false;
        self.can_move_left = false;
        self.can_move_right = false;
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Background;

fn startup(mut commands: Commands) {
    commands.spawn((Name::new("Background"), Background, Sprite::default()));
    commands.spawn((
        Name::new("Rooms"),
        Visibility::Visible,
        children![
            bathroom::room(),
            bathroom_washer::room(),
            bedroom::room(),
            bedroom_bed::room(),
            hall::room(),
            kitchen::room(),
            kitchen_fridge::room(),
            kitchen_window::room(),
        ],
    ));
}

fn navigate(
    input: Single<&ActionState<Action>>,
    room_query: Query<&RoomComponent>,
    current_room: Res<State<Room>>,
    mut next_room: ResMut<NextState<Room>>,
) {
    let Some(room) = room_query.iter().find(|r| r.is(**current_room)) else {
        #[cfg(debug_assertions)]
        println!(
            "Could not find the component for {:?}; existing rooms: {:?}",
            **current_room,
            room_query.iter().map(|r| r.this_room).collect::<Vec<_>>()
        );
        return;
    };

    let direction = if input.just_pressed(&Action::Forward) && room.can_move_forward {
        room.forward_room
    } else if input.just_pressed(&Action::Back) && room.can_move_back {
        room.back_room
    } else if input.just_pressed(&Action::Left) && room.can_move_left {
        room.left_room
    } else if input.just_pressed(&Action::Right) && room.can_move_right {
        room.right_room
    } else {
        None
    };

    if let Some(room) = direction {
        #[cfg(debug_assertions)]
        if room_query.iter().find(|r| r.is(room)).is_none() {
            warn!(
                "Tried to enter room {:?} that does not have an implementation",
                room
            );
            return;
        };

        next_room.set(room);
    }
}

fn show_room(current_room: Res<State<Room>>, query: Query<(&RoomComponent, &mut Visibility)>) {
    for (room, mut visibility) in query {
        *visibility = if room.is(**current_room) {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn show_background(
    current_screen: Res<State<Screen>>,
    mut visibility: Single<&mut Visibility, With<Background>>,
) {
    **visibility = if **current_screen == Screen::Gameplay {
        Visibility::Visible
    } else {
        Visibility::Hidden
    }
}

#[cfg(debug_assertions)]
fn room_check(query: Query<&RoomComponent>) {
    let mut used_rooms: Vec<Room> = Vec::new();
    for room in query {
        let room = room.this_room;
        if used_rooms.contains(&room) {
            warn!("Found duplicate room: {:?}", room);
        } else {
            used_rooms.push(room);
        }
    }
}
