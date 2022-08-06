use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioChannel, AudioPlugin, AudioSource};
use iyes_loopless::prelude::*;

use crate::ApplicationState;

pub struct GameAudioPlugin;

#[derive(Component, Default, Clone)]
struct Background;

#[derive(PartialEq)]
enum BackgroundMusicState {
    /// Music playing
    // Playing,

    /// Music has been paused
    Paused,

    /// Music has been stopped or never played before
    Stopped,
}

struct AudioState {
    bg_handle: Handle<AudioSource>,
    bg_state: BackgroundMusicState,

    volume: f32,
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(ApplicationState::Game, start_bg_music)
            .add_exit_system(ApplicationState::Game, stop_bg_music)
            .add_plugin(AudioPlugin)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_audio)
            .add_audio_channel::<Background>();
    }
}

fn start_bg_music(background_audio: Res<AudioChannel<Background>>, audio_state: Res<AudioState>) {
    match audio_state.bg_state {
        // If the song is stopped or never played before we just need to start it
        BackgroundMusicState::Stopped => {
            background_audio.play_looped(audio_state.bg_handle.clone());
            background_audio.set_volume(audio_state.volume);
        }

        // We don't need to do anything if the song is already playing
        // BackgroundMusicState::Playing => {}

        // We have to resume the music then
        BackgroundMusicState::Paused => {
            background_audio.resume();
        }
    }
}

fn stop_bg_music(
    background_audio: Res<AudioChannel<Background>>,
    mut audio_state: ResMut<AudioState>,
) {
    background_audio.pause();
    audio_state.bg_state = BackgroundMusicState::Paused;
}

fn load_audio(mut commands: Commands, assets: Res<AssetServer>) {
    let bgm_handle = assets.load("audio/deepwater-ruins.ogg");

    commands.insert_resource(AudioState {
        bg_handle: bgm_handle,
        bg_state: BackgroundMusicState::Stopped,
        volume: 0.5,
    });
}
