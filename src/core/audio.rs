use bevy::audio::{
    AudioPlayer, AudioSink, AudioSinkPlayback, AudioSource, PlaybackSettings, Volume,
};
use bevy::prelude::*;

use crate::app::states::GameState;
use crate::core::config::GameConfig;

#[derive(Resource, Clone)]
pub struct AudioAssets {
    pub bounce: Handle<AudioSource>,
    pub brick_break: Handle<AudioSource>,
    pub menu_music: Handle<AudioSource>,
    pub gameplay_music: Handle<AudioSource>,
}

#[derive(Component)]
pub struct BackgroundMusic;

#[derive(Component, Clone, Copy, Eq, PartialEq)]
pub enum MusicTrack {
    Menu,
    Gameplay,
}

pub fn load_audio_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioAssets {
        bounce: asset_server.load("sounds/bounce.ogg"),
        brick_break: asset_server.load("sounds/break.ogg"),
        menu_music: asset_server.load("music/menu.ogg"),
        gameplay_music: asset_server.load("music/gameplay.ogg"),
    });
}

pub fn sync_background_music(
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    config: Res<GameConfig>,
    audio_assets: Res<AudioAssets>,
    mut music_query: Query<(Entity, &MusicTrack, Option<&mut AudioSink>), With<BackgroundMusic>>,
) {
    let target_track = music_track_for_state(*game_state.get());
    let target_volume = Volume::Linear(config.music_volume.clamp(0.0, 1.0));
    let should_refresh = game_state.is_changed();
    let should_update_volume = config.is_changed();

    if !should_refresh && !should_update_volume && !music_query.is_empty() {
        return;
    }

    let mut matching_entity = None;

    for (entity, track, sink) in music_query.iter_mut() {
        if *track == target_track {
            matching_entity = Some(entity);
            if should_update_volume {
                if let Some(mut sink) = sink {
                    sink.set_volume(target_volume);
                }
            }
        } else {
            commands.entity(entity).despawn();
        }
    }

    if matching_entity.is_some() {
        return;
    }

    commands.spawn((
        AudioPlayer::new(music_source_for_track(&audio_assets, target_track).clone()),
        PlaybackSettings::LOOP.with_volume(target_volume),
        BackgroundMusic,
        target_track,
    ));
}

pub fn play_sfx(
    commands: &mut Commands,
    source: &Handle<AudioSource>,
    config: &GameConfig,
    volume_scale: f32,
) {
    let volume = (config.sfx_volume * volume_scale).clamp(0.0, 1.0);
    if volume <= 0.0 {
        return;
    }

    commands.spawn((
        AudioPlayer::new(source.clone()),
        PlaybackSettings::DESPAWN.with_volume(Volume::Linear(volume)),
    ));
}

fn music_track_for_state(state: GameState) -> MusicTrack {
    match state {
        GameState::MainMenu | GameState::HighScores | GameState::Settings => MusicTrack::Menu,
        GameState::Playing
        | GameState::Paused
        | GameState::LevelComplete
        | GameState::GameOver
        | GameState::Victory => MusicTrack::Gameplay,
    }
}

fn music_source_for_track(audio_assets: &AudioAssets, track: MusicTrack) -> &Handle<AudioSource> {
    match track {
        MusicTrack::Menu => &audio_assets.menu_music,
        MusicTrack::Gameplay => &audio_assets.gameplay_music,
    }
}
