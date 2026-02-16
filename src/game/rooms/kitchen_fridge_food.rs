use bevy::{ecs::{observer, system::command}, prelude::*};
use bevy_bundled_observers::observers;

use crate::{audio::{PlaySfx, Sfxlib}, game::enemies::doordash::{Food, HeldFood}};

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

    #[dependency]
    apple_thought: Handle<Image>,
    #[dependency]
    burrito_thought: Handle<Image>,
    #[dependency]
    italian_sandwich_thought: Handle<Image>,
    #[dependency]
    salami_sandwich_thought: Handle<Image>,
    #[dependency]
    tralala_thought: Handle<Image>,
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

            apple_thought: assets.load("images/rooms/prop/food/apple_thought.png"),
            burrito_thought: assets.load("images/rooms/prop/food/burrito_thought.png"),
            italian_sandwich_thought: assets
                .load("images/rooms/prop/food/italian_sandwich_thought.png"),
            salami_sandwich_thought: assets
                .load("images/rooms/prop/food/salami_sandwich_thought.png"),
            tralala_thought: assets.load("images/rooms/prop/food/tralala_thought.png"),
        }
    }
}

impl FoodAssets {
    pub fn get_thought(&self, food: Food) -> Handle<Image> {
        match food {
            Food::Apple => self.apple_thought.clone(),
            Food::Burrito => self.burrito_thought.clone(),
            Food::ItalianSandwich => self.italian_sandwich_thought.clone(),
            Food::SalamiSandwich => self.salami_sandwich_thought.clone(),
            Food::Tralala => self.tralala_thought.clone(),
        }
    }

    pub fn get(&self, food: Food) -> Handle<Image> {
        match food {
            Food::Apple => self.apple.clone(),
            Food::Burrito => self.burrito.clone(),
            Food::ItalianSandwich => self.italian_sandwich.clone(),
            Food::SalamiSandwich => self.salami_sandwich.clone(),
            Food::Tralala => self.tralala.clone(),
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct FridgeFood(Food);

#[rustfmt::skip]
fn food(food: Food, assets: &FoodAssets) -> impl Bundle {
    (
        FridgeFood(food),
        Visibility::default(),
        Sprite::from_image(assets.get(food)),
        Pickable::default(),
        observers![pick_food],
    )
}

pub(super) fn all_food(assets: &FoodAssets) -> impl Bundle {
    children![
        food(Food::Apple, assets),
        food(Food::Burrito, assets),
        food(Food::ItalianSandwich, assets),
        food(Food::SalamiSandwich, assets),
        food(Food::Tralala, assets),
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
    sfx_asset: Res<Sfxlib>,
    mut commands: Commands,
) {
    let Ok(food) = food_query.get(ev.event_target()) else {
        return;
    };

    held_food.0 = Some(food.0);
    commands.play_volume_sfx(sfx_asset.item_pickup.clone(), 0.5);
}
