use bevy::prelude::*;

use crate::asset_tracking::LoadResource;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<Sfxlib>();
    app.add_systems(
        Update,
        apply_global_volume.run_if(resource_changed::<GlobalVolume>),
    );
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "music" category (e.g. global background music, soundtrack).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Music;

/// A music audio instance.
pub fn music(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::LOOP, Music)
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "sound effect" category (e.g. footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SoundEffect;

/// A sound effect audio instance.
pub fn sound_effect(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::DESPAWN, SoundEffect)
}

/// [`GlobalVolume`] doesn't apply to already-running audio entities, so this system will update them.
fn apply_global_volume(
    global_volume: Res<GlobalVolume>,
    mut audio_query: Query<(&PlaybackSettings, &mut AudioSink)>,
) {
    for (playback, mut sink) in &mut audio_query {
        sink.set_volume(global_volume.volume * playback.volume);
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct Sfxlib {
    #[dependency]
    pub curtains_close_1: Handle<AudioSource>,
    #[dependency]
    pub curtains_close_2: Handle<AudioSource>,
    #[dependency]
    pub curtains_close_3: Handle<AudioSource>,
}

impl FromWorld for Sfxlib {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            curtains_close_1: assets.load("audio/sound_effects/curtains_close_1.ogg"),
            curtains_close_2: assets.load("audio/sound_effects/curtains_close_2.ogg"),
            curtains_close_3: assets.load("audio/sound_effects/curtains_close_3.ogg"),
        }
    }
}



pub trait PlaySfx {
    fn play_sfx(&mut self, handle: Handle<AudioSource>, settings: PlaybackSettings);
    fn play_simple_sfx(&mut self, handle: Handle<AudioSource>);
    fn play_loop_sfx(&mut self, handle: Handle<AudioSource>);
}

impl<'w, 's> PlaySfx for Commands<'w, 's> {
    fn play_sfx(&mut self, handle: Handle<AudioSource>, settings: PlaybackSettings) {
        self.spawn((AudioPlayer(handle), settings, SoundEffect));
    }
    fn play_simple_sfx(&mut self, handle: Handle<AudioSource>) {
        self.play_sfx(handle, PlaybackSettings::DESPAWN);
    }
    fn play_loop_sfx(&mut self, handle: Handle<AudioSource>) {
        self.play_sfx(handle, PlaybackSettings::LOOP);
    }
}