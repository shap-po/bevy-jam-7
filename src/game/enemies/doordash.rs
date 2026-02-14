use crate::game::{enemies::{Difficulty, Enemy, EnemyTicked, EnemyType, doordash}, events::GameOver};
use bevy::{prelude::*, transform::components};
use rand::{Rng, rng};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_opportunity);
    app.add_observer(doordash_complete);
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
    mut command: Commands,

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
                command.trigger(GameOver::Loose(EnemyType::Doordash));
                Doordash::Death
            }
            else {
                Doordash::Waiting{state_counter: state_counter + 1, food_want}
            }
        },
        Doordash::Death => Doordash::Death,
    }
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct FoodPlayer(Option<Food>);



#[derive(Event, Reflect, Debug, Default)]
pub struct FoodBrought;


fn doordash_complete(
    event: On<FoodBrought>,
    mut doordash: Single<&mut Doordash>,
    mut command: Commands,
    mut food_player: ResMut<FoodPlayer>,
){
    let Doordash::Waiting{ state_counter, food_want } = **doordash else {return;};
    let Some(food_player_unpack) = food_player.0 else {return;};

    if food_player_unpack == food_want{
        **doordash = Doordash::Away(0);
        food_player.0 = None;
    }
    else{
        command.trigger(GameOver::Loose(EnemyType::Doordash));
        **doordash = Doordash::Death;
    }
}