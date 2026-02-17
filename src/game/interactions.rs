use bevy::{
    platform::collections::HashSet,
    prelude::*,
    window::{CursorIcon, SystemCursorIcon},
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(SpritePickingSettings {
        require_markers: false,
        picking_mode: SpritePickingMode::AlphaThreshold(0.01),
    });

    app.init_resource::<HoveredEntities>();
    app.add_observer(update_hovered::<Pointer<Over>>(true));
    app.add_observer(update_hovered::<Pointer<Out>>(false));
    app.add_systems(Startup, init_cursor_icon);
    app.add_systems(Update, update_cursor);
}

fn init_cursor_icon(mut commands: Commands, window: Single<Entity, With<Window>>) {
    commands.entity(*window).insert(CursorIcon::default());
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
struct HoveredEntities(HashSet<Entity>);

fn update_hovered<E>(add: bool) -> impl Fn(On<E>, ResMut<HoveredEntities>)
where
    E: EntityEvent + std::fmt::Debug + Clone + Reflect,
{
    move |event, mut hovered| {
        if add {
            hovered.0.insert(event.event_target());
        } else {
            hovered.0.remove(&event.event_target());
        }
    }
}

fn update_cursor(
    mut hovered: ResMut<HoveredEntities>,
    hover_cursors: Query<&ViewVisibility, With<Pickable>>,
    mut cursor_icon: Single<&mut CursorIcon>,
) {
    hovered.0.retain(|entity| match hover_cursors.get(*entity) {
        Ok(visibility) => visibility.get(),
        Err(_) => false,
    });

    **cursor_icon = if hovered.0.is_empty() {
        default()
    } else {
        CursorIcon::System(SystemCursorIcon::Pointer)
    };
}
