use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioChannel, AudioPlugin, AudioSource};
use iyes_loopless::prelude::*;

use crate::ron_parsers::Settings;
use crate::ApplicationState;

pub struct GameAudioPlugin;

#[derive(Component, Default, Clone)]
struct Background;

#[derive(PartialEq, Debug)]
enum BackgroundMusicState {
    /// Music playing
    Playing,

    /// Music has been paused
    Paused,

    /// Music has been stopped or never played before
    Stopped,
}

#[derive(Debug)]
pub struct AudioState {
    // Describes that audio is turned `on` or `off` (by default it's `true`)
    pub state: bool,

    bg_handle: Handle<AudioSource>,
    bg_state: BackgroundMusicState,

    pub volume: i8,
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(ApplicationState::Game, start_bg_music)
            .add_exit_system(ApplicationState::Game, stop_bg_music)
            .add_plugin(AudioPlugin)
            .add_startup_system_to_stage(StartupStage::PostStartup, load_audio)
            .add_system(sync_audio_state_and_settings)
            .add_audio_channel::<Background>();
    }
}

fn start_bg_music(
    background_audio: Res<AudioChannel<Background>>,
    mut audio_state: ResMut<AudioState>,
) {
    if audio_state.state {
        background_audio.set_volume(audio_state.volume as f32 / 10.0);

        match audio_state.bg_state {
            // If the song is stopped or never played before we just need to start it
            BackgroundMusicState::Stopped => {
                background_audio.play_looped(audio_state.bg_handle.clone());
                audio_state.bg_state = BackgroundMusicState::Playing;
            }

            // We don't need to do anything if the song is already playing
            BackgroundMusicState::Playing => {}

            // We have to resume the music then
            BackgroundMusicState::Paused => {
                background_audio.resume();
                audio_state.bg_state = BackgroundMusicState::Playing;
            }
        }
    } else {
        background_audio.stop();
    }
}

/// Sync audio state when its change with global settings
fn sync_audio_state_and_settings(audio_state: Res<AudioState>, mut settings: ResMut<Settings>) {
    if audio_state.is_changed() {
        settings.audio.state = audio_state.state;
        settings.audio.volume = audio_state.volume;
    }
}

fn stop_bg_music(
    background_audio: Res<AudioChannel<Background>>,
    mut audio_state: ResMut<AudioState>,
) {
    // If the audio is turned on we just need to pause the music
    // Otherwise - stop the music and change the BackgroundMusicState
    if audio_state.state {
        background_audio.pause();
        audio_state.bg_state = BackgroundMusicState::Paused;
    } else {
        background_audio.stop();
        audio_state.bg_state = BackgroundMusicState::Stopped;
    }
}

/// Loads audio. Initialized in PostStartup stage to be sure
///  that all `add_startup_systems` has been initialized
fn load_audio(mut commands: Commands, assets: Res<AssetServer>, settings: Res<Settings>) {
    let bgm_handle = assets.load("audio/deepwater-ruins.ogg");

    commands.insert_resource(AudioState {
        // By default turn on the audio. Later we should
        //  read this information from the disk (because maybe the user)
        //  decided to turn the audio off
        state: settings.audio.state,

        bg_handle: bgm_handle,
        bg_state: BackgroundMusicState::Stopped,
        volume: settings.audio.volume,
    });
}
