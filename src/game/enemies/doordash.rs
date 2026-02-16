use crate::game::{
    enemies::{Difficulty, Enemy, EnemyTicked, EnemyType, doordash},
    events::GameOver,
};
use bevy::{prelude::*, transform::components};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<HeldFood>();
    app.add_observer(handle_opportunity);
    app.add_observer(doordash_complete);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub enum Doordash {
    Away(i8),
    Waiting { state_counter: i8, food_want: Food },
    Death,
}

#[derive(Reflect, Debug, Default, PartialEq, Clone, Copy)]
pub enum Food {
    #[default]
    Apple,
    Burrito,
    ItalianSandwich,
    SalamiSandwich,
    Tralala,
}

impl Food {
    fn rand() -> Self {
        match rand::random_range(0..=4) {
            0 => Food::Apple,
            1 => Food::Burrito,
            2 => Food::ItalianSandwich,
            3 => Food::SalamiSandwich,
            _ => Food::Tralala,
        }
    }
}

pub fn doordash(difficulty: i8) -> impl Bundle {
    (
        Name::new("Doordash"),
        Doordash::Away(0),
        Difficulty(difficulty),
        Enemy,
    )
}

const MAX_STATES_AWAY: i8 = 2; // todo: replace with difficulty scaling
const MAX_STATES_WAITING: i8 = 5; // todo: replace with difficulty scaling

fn handle_opportunity(
    event: On<EnemyTicked>,
    doordash_query: Single<(Entity, &mut Doordash)>,
    mut command: Commands,
) {
    let (entity, mut doordash) = doordash_query.into_inner();
    if entity != event.event_target() {
        return;
    }

    *doordash = match *doordash {
        Doordash::Away(i) => {
            if i > MAX_STATES_AWAY {
                Doordash::Waiting {
                    state_counter: 0,
                    food_want: Food::rand(),
                }
            } else {
                Doordash::Away(i + 1)
            }
        }
        Doordash::Waiting {
            state_counter,
            food_want,
        } => {
            if state_counter > MAX_STATES_WAITING {
                command.trigger(GameOver::Death(EnemyType::Doordash));
                Doordash::Death
            } else {
                Doordash::Waiting {
                    state_counter: state_counter + 1,
                    food_want,
                }
            }
        }
        Doordash::Death => Doordash::Death,
    }
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct HeldFood(pub Option<Food>);

#[derive(Event, Reflect, Debug, Default)]
pub struct FoodBrought;

fn doordash_complete(
    event: On<FoodBrought>,
    mut doordash: Single<&mut Doordash>,
    mut command: Commands,
    mut held_food: ResMut<HeldFood>,
) {
    let Doordash::Waiting {
        state_counter,
        food_want,
    } = **doordash
    else {
        return;
    };
    let Some(food) = held_food.0 else {
        return;
    };

    #[cfg(debug_assertions)]
    println!("Gave {:?} to doordash, he wants {:?}", food, food_want);

    held_food.0 = None;
    if food == food_want {
        **doordash = Doordash::Away(0);
    } else {
        command.trigger(GameOver::Death(EnemyType::Doordash));
        **doordash = Doordash::Death;
    }
}
