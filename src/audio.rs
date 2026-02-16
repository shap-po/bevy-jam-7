use bevy::{audio::Volume, prelude::*};

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
    #[dependency]
    pub curtains_open_1: Handle<AudioSource>,
    #[dependency]
    pub curtains_open_2: Handle<AudioSource>,
    #[dependency]
    pub curtains_open_3: Handle<AudioSource>,
    #[dependency]
    pub fridge_loop: Handle<AudioSource>,
    #[dependency]
    pub fridge_open_1: Handle<AudioSource>,
    #[dependency]
    pub fridge_close_1: Handle<AudioSource>,
    #[dependency]
    pub hall_ambient_clock: Handle<AudioSource>,
    #[dependency]
    pub window_ambient_forest: Handle<AudioSource>,
    #[dependency]
    pub door_open_and_close_1: Handle<AudioSource>,
    #[dependency]
    pub player_run_1: Handle<AudioSource>,
    #[dependency]
    pub player_run_2: Handle<AudioSource>,
    #[dependency]
    pub player_run_3: Handle<AudioSource>,
    #[dependency]
    pub spooky_sfx_1: Handle<AudioSource>,
    #[dependency]
    pub blanket_1: Handle<AudioSource>,
    #[dependency]
    pub blanket_2: Handle<AudioSource>,
    #[dependency]
    pub blanket_3: Handle<AudioSource>,
    #[dependency]
    pub blanket_4: Handle<AudioSource>,
    #[dependency]
    pub washer_ambient_loop: Handle<AudioSource>,
    #[dependency]
    pub washer_outro: Handle<AudioSource>,
    #[dependency]
    pub washer_beep: Handle<AudioSource>,
    #[dependency]
    pub window_broke_in: Handle<AudioSource>,
    
    
}

impl FromWorld for Sfxlib {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            curtains_close_1: assets.load("audio/sound_effects/curtains_close_1.ogg"),
            curtains_close_2: assets.load("audio/sound_effects/curtains_close_2.ogg"),
            curtains_close_3: assets.load("audio/sound_effects/curtains_close_3.ogg"),
            curtains_open_1: assets.load("audio/sound_effects/curtains_open_1.ogg"),
            curtains_open_2: assets.load("audio/sound_effects/curtains_open_2.ogg"),
            curtains_open_3: assets.load("audio/sound_effects/curtains_open_3.ogg"),
            fridge_loop: assets.load("audio/sound_effects/fridge_loop.ogg"),
            fridge_open_1: assets.load("audio/sound_effects/fridge_open_1.ogg"),
            fridge_close_1: assets.load("audio/sound_effects/fridge_close_1.ogg"),
            hall_ambient_clock: assets.load("audio/sound_effects/hall_ambient_clock.ogg"),
            window_ambient_forest: assets.load("audio/sound_effects/window_ambience.ogg"),
            door_open_and_close_1: assets.load("audio/sound_effects/door_open_and_close_1.ogg"),
            player_run_1: assets.load("audio/sound_effects/run_1.ogg"),
            player_run_2: assets.load("audio/sound_effects/run_2.ogg"),
            player_run_3: assets.load("audio/sound_effects/run_3.ogg"),
            spooky_sfx_1: assets.load("audio/sound_effects/spooky_sfx_1-doorcreacking.ogg"),
            blanket_1: assets.load("audio/sound_effects/blanket_1.ogg"),
            blanket_2: assets.load("audio/sound_effects/blanket_2.ogg"),
            blanket_3: assets.load("audio/sound_effects/blanket_3.ogg"),
            blanket_4: assets.load("audio/sound_effects/blanket_4.ogg"),
            washer_ambient_loop: assets.load("audio/sound_effects/washer_loop.ogg"),
            washer_outro: assets.load("audio/sound_effects/washer_outro.ogg"),
            washer_beep: assets.load("audio/sound_effects/washer_beep.ogg"),
            window_broke_in: assets.load("audio/sound_effects/window_broke_in.ogg"),
        }
    }
}
impl Sfxlib {
    pub fn rand_player_run(&self) -> Handle<AudioSource>{
        match rand::random_range(0..=2) {
            0 => self.player_run_1.clone(),
            1 => self.player_run_2.clone(),
            _ => self.player_run_3.clone(),
            }
    }
    pub fn rand_curtains_open(&self) -> Handle<AudioSource>{
        match rand::random_range(0..=2) {
            0 => self.curtains_open_1.clone(),
            1 => self.curtains_open_2.clone(),
            _ => self.curtains_open_3.clone(),
            }
    }
    pub fn rand_curtains_close(&self) -> Handle<AudioSource>{
        match rand::random_range(0..=2) {
            0 => self.curtains_close_1.clone(),
            1 => self.curtains_close_2.clone(),
            _ => self.curtains_close_3.clone(),
            }
    }
    pub fn rand_spooky_sfx(&self) -> Handle<AudioSource>{
        match rand::random_range(0..=1) {
            0 => self.spooky_sfx_1.clone(),
            _ => self.spooky_sfx_1.clone(),
            }
    }
    pub fn rand_door_open_and_close(&self) -> Handle<AudioSource>{
        match rand::random_range(0..=0) {
            _ => self.door_open_and_close_1.clone(),
            }
    }
    pub fn rand_fridge_close(&self) -> Handle<AudioSource>{
        match rand::random_range(0..=0) {
            _ => self.fridge_close_1.clone(),
            }
    }
    pub fn rand_fridge_open(&self) -> Handle<AudioSource>{
        match rand::random_range(0..=0) {
            _ => self.fridge_open_1.clone(),
            }
    }
    pub fn rand_blanket(&self) -> Handle<AudioSource>{
        match rand::random_range(0..=3) {
            0 => self.blanket_1.clone(),
            1 => self.blanket_2.clone(),
            2 => self.blanket_3.clone(),
            _ => self.blanket_4.clone(),
            }
    }
}


pub trait PlaySfx {
    fn play_volume_sfx(&mut self, handle: Handle<AudioSource>, vol: f32);
    fn play_sfx(&mut self, handle: Handle<AudioSource>, settings: PlaybackSettings);
    fn play_simple_sfx(&mut self, handle: Handle<AudioSource>);
    fn play_loop_sfx(&mut self, handle: Handle<AudioSource>, vol: f32, state: impl States);
}

impl<'w, 's> PlaySfx for Commands<'w, 's> {
    fn play_sfx(&mut self, handle: Handle<AudioSource>, settings: PlaybackSettings) {
        self.spawn((AudioPlayer(handle), settings, SoundEffect));
    }
    fn play_simple_sfx(&mut self, handle: Handle<AudioSource>) {
        self.play_sfx(handle, PlaybackSettings::DESPAWN);
    }
    fn play_volume_sfx(&mut self, handle: Handle<AudioSource>, vol: f32) {
        self.play_sfx(handle, PlaybackSettings::DESPAWN.with_volume(Volume::Linear(vol)));
    }
    fn play_loop_sfx(&mut self, handle: Handle<AudioSource>, vol: f32, state: impl States) {
        //DespawnOnExit(Menu::Credits),
        self.spawn((AudioPlayer(handle), DespawnOnExit(state), PlaybackSettings::LOOP.with_volume(Volume::Linear(vol)), SoundEffect));
        //self.play_sfx(handle, PlaybackSettings::LOOP.with_volume(Volume::Linear(vol)));
    }
}

