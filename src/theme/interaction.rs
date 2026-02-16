use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(apply_background_palette::<Pointer<Click>>(|p| p.pressed))
        .add_observer(apply_background_palette::<Pointer<Release>>(|p| p.hovered))
        .add_observer(apply_background_palette::<Pointer<Over>>(|p| p.hovered))
        .add_observer(apply_background_palette::<Pointer<Out>>(|p| p.none));

    app.add_observer(apply_text_color_palette::<Pointer<Click>>(|p| p.pressed))
        .add_observer(apply_text_color_palette::<Pointer<Release>>(|p| p.hovered))
        .add_observer(apply_text_color_palette::<Pointer<Over>>(|p| p.hovered))
        .add_observer(apply_text_color_palette::<Pointer<Out>>(|p| p.none));
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct BackgroundPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct TextColorPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

fn apply_background_palette<E>(
    color_getter: impl Fn(&BackgroundPalette) -> Color,
) -> impl Fn(On<E>, Query<(&BackgroundPalette, &mut BackgroundColor)>)
where
    E: EntityEvent + std::fmt::Debug + Clone + Reflect,
{
    move |event, mut query| {
        let Ok((palette, mut bg)) = query.get_mut(event.event_target()) else {
            return;
        };

        *bg = color_getter(palette).into();
    }
}

fn apply_text_color_palette<E>(
    color_getter: impl Fn(&TextColorPalette) -> Color,
) -> impl Fn(On<E>, Query<(&TextColorPalette, &mut TextColor)>)
where
    E: EntityEvent + std::fmt::Debug + Clone + Reflect,
{
    move |event, mut query| {
        let Ok((palette, mut text)) = query.get_mut(event.event_target()) else {
            return;
        };

        *text = color_getter(palette).into();
    }
}
