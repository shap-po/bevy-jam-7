use bevy::{ecs::observer, prelude::*};
use bevy_bundled_observers::observers;

use crate::game::enemies::doordash::{Food, HeldFood};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<FoodAssets>();

    app.add_systems(Update, hide_inventory_food);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct FoodAssets {
    #[dependency]
    apple: Handle<Image>,
    #[dependency]
    burrito: Handle<Image>,
    #[dependency]
    italian_sandwich: Handle<Image>,
    #[dependency]
    salami_sandwich: Handle<Image>,
    #[dependency]
    tralala: Handle<Image>,
}

impl FromWorld for FoodAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            apple: assets.load("images/rooms/prop/food/apple.png"),
            burrito: assets.load("images/rooms/prop/food/burrito.png"),
            italian_sandwich: assets.load("images/rooms/prop/food/italian_sandwich.png"),
            salami_sandwich: assets.load("images/rooms/prop/food/salami_sandwich.png"),
            tralala: assets.load("images/rooms/prop/food/tralala.png"),
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct FridgeFood(Food);

#[rustfmt::skip]
fn food(food: Food, image: Handle<Image>) -> impl Bundle {
    (
        FridgeFood(food),
        Visibility::default(),
        Sprite::from_image(image),
        Pickable::default(),
        observers![pick_food],
    )
}

pub(super) fn all_food(assets: &FoodAssets) -> impl Bundle {
    children![
        food(Food::Apple, assets.apple.clone()),
        food(Food::Burrito, assets.burrito.clone()),
        food(Food::ItalianSandwich, assets.italian_sandwich.clone()),
        food(Food::SalamiSandwich, assets.salami_sandwich.clone()),
        food(Food::Tralala, assets.tralala.clone()),
    ]
}

fn hide_inventory_food(
    food_query: Query<(&mut Visibility, &FridgeFood)>,
    held_food: Res<HeldFood>,
) {
    for (mut visibility, food) in food_query {
        *visibility = if Some(food.0) == held_food.0 {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
    }
}

fn pick_food(
    ev: On<Pointer<Press>>,
    food_query: Query<&FridgeFood>,
    mut held_food: ResMut<HeldFood>,
) {
    let Ok(food) = food_query.get(ev.event_target()) else {
        return;
    };

    held_food.0 = Some(food.0);
}
