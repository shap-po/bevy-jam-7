use crate::game::enemies::{Difficulty, Enemy, EnemyTicked};
use bevy::{prelude::*, transform::components};
use rand::{Rng, rng};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<FoodBrought>();
    app.add_observer(handle_opportunity);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
enum Doordash{
    Away(i8),
    Waiting{state_counter: i8, food_want: Food},
    Death,
}

#[derive(Reflect, Debug, Default, PartialEq, Clone, Copy)]
enum Food{
    #[default] Cheese,
    Pizza,
}

impl Food{
    fn rand() -> Self{
        let mut  rng = rand::rng();
        match rng.random_range(0..=1){
            0 => Food::Cheese,
            _ => Food::Pizza,
        }
    }
}

pub fn doordash(difficulty: i8) -> impl Bundle {
    (
        Name::new("Doordash"),
        Doordash::Away(0),
        Difficulty::new(difficulty),
        Enemy,
    )
}

const MAXSTATESAWAY: i8 = 2; // todo: replace with difficulty scaling
const MAXSTATESWAITING: i8 = 5; // todo: replace with difficulty scaling

fn handle_opportunity(
    event: On<EnemyTicked>,
    doordash_query: Single<(Entity, &mut Doordash)>,
    food_brought: Res<FoodBrought>,

) {
    let (entity, mut doordash) = doordash_query.into_inner();
    if entity != event.event_target() {
        return;
    }

    *doordash = match *doordash {
        Doordash::Away(i) => {
            if i > MAXSTATESAWAY {
                Doordash::Waiting{state_counter :0, food_want: Food::rand()}
            }
            else {
                Doordash::Away(i + 1)
            }
        },
        Doordash::Waiting{state_counter , food_want} => {
            if state_counter > MAXSTATESWAITING {
                Doordash::Death
            }
            else if food_brought.0 == food_want{
                Doordash::Away(0)
            }
            else {
                Doordash::Waiting{state_counter: state_counter + 1, food_want}
            }
        },
        Doordash::Death => todo!(),
    }
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct FoodBrought(Food);