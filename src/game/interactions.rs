use bevy::{prelude::*, state::commands, window::PrimaryWindow};

use crate::AppSystems;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(SpritePickingSettings {
        require_markers: false,
        picking_mode: SpritePickingMode::AlphaThreshold(0.01),
    });

    app.add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    commands.add_observer(observe_picking::<Pointer<Over>>());
    commands.add_observer(observe_picking::<Pointer<Out>>());
    commands.add_observer(observe_picking::<Pointer<Press>>());
    commands.add_observer(observe_picking::<Pointer<Release>>());
}

fn observe_picking<E>() -> impl Fn(On<E>, Query<&mut Sprite>)
where
    E: EntityEvent + std::fmt::Debug + Clone + Reflect,
{
    move |ev, sprites| {
        let Ok(sprite) = sprites.get(ev.event_target()) else {
            return;
        };
        println!("Observed {:?} on {:?}", ev.event_key(), sprite.image.path());
    }
}
