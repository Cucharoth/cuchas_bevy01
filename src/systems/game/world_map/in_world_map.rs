use crate::components::*;
use crate::prelude::*;
use bevy::audio::*;
use bevy::audio::{self, PlaybackMode};
use bevy::render::camera::ScalingMode;
use bevy::window::PrimaryWindow;
use resources::*;
use world_map::components::*;
use world_map::*;

use super::enemy::EnemyPlugin;
use super::player::PlayerPlugin;

pub const ENEMY_SIZE: (f32, f32) = (90.0, 100.0);
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size
pub const NUMBER_OF_ENEMIES: usize = 4;

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, EnemyPlugin))
            // on enter
            .add_systems(OnEnter(InGameState::WorldMap), load_background)
            .add_systems(OnExit(InGameState::WorldMap), despawn_background)
            .add_systems(
                Update,
                (enemy_hit_player,)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Running))
                    .run_if(in_state(InGameState::WorldMap)),
            )
            .add_systems(
                Update,
                (timer_after_collission_check, global_timer_update)
                    .run_if(in_state(InGameState::WorldMap)),
            )
            //audio handle
            .add_systems(OnEnter(InGameState::WorldMap), play_map_music)
            .add_systems(
                OnExit(InGameState::WorldMap),
                stop_map_music.run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), stop_map_music)
            .add_systems(
                OnEnter(GameState::Paused),
                volume_in_pause.run_if(in_state(InGameState::WorldMap)),
            )
            .add_systems(
                OnEnter(GameState::Running),
                volume_in_running.run_if(in_state(InGameState::WorldMap)),
            );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);

    /*let texture = asset_server.load("Enemy/devil.png");

    //enemy
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 70.0)),
                ..default()
            },
            texture,
            ..default()
        },
        Enemy {
            damage: 10,
        },

    ));*/
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 999.0),
        ..default()
    });
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<WorldMapPlayer>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    audio_control: Query<&AudioSink, With<WorldMapTheme>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    assest_server: Res<AssetServer>,
    mut player_status: ResMut<PlayerStatus>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            //let player_radius = PLAYER_SIZE / 2.0;
            //let enemy_radius = ENEMY_SIZE.0 / 2.0;
            let collision_distance = 50.;
            if distance < collision_distance {
                player_status.transform = *player_transform;
                commands.insert_resource(PlayerEntity {
                    entity: player_entity,
                });
                commands.insert_resource(EnemyEntityCollisioned {
                    entity: enemy_entity,
                });
                let monster_found_audio = assest_server.load("audio/effects/monster_found.wav");
                commands.spawn(AudioBundle {
                    source: monster_found_audio,
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Once,
                        ..default()
                    },
                    ..Default::default()
                });
                commands.init_resource::<AfterEnemyCollisionTimer>();
                next_game_state.set(GameState::Transition);
                audio_control.get_single().unwrap().set_volume(0.4);
            }
        }
    }
}

pub fn global_timer_update(
    mut collision_timer: Option<ResMut<AfterEnemyCollisionTimer>>,
    time: Res<Time>,
) {
    if let Some(collision_timer) = &mut collision_timer {
        let time_value = collision_timer.timer.tick(time.delta());
        //println!("{:?}", time_value);
    }
}

pub fn timer_after_collission_check(
    mut commands: Commands,
    collision_timer: Option<Res<AfterEnemyCollisionTimer>>,
    mut next_state: ResMut<NextState<InGameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Some(timer) = &collision_timer {
        if timer.timer.finished() {
            commands.remove_resource::<AfterEnemyCollisionTimer>();
            next_state.set(InGameState::Fight);
            next_game_state.set(GameState::Running);
        }
    }
}

fn play_map_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    let theme = asset_server.load("audio/2-19 Venus Lighthouse.wav");
    commands.spawn((
        AudioBundle {
            source: theme,
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::Relative(VolumeLevel::new(0.6)),
                ..Default::default()
            },
        },
        WorldMapTheme,
    ));
}

fn stop_map_music(
    mut commands: Commands,
    world_map_theme_query: Query<(Entity, &AudioSink), With<WorldMapTheme>>,
) {
    if let Ok((theme_entity, music_control)) = world_map_theme_query.get_single() {
        music_control.stop();
        commands.entity(theme_entity).despawn();
    }
}

fn volume_in_pause(audio_query: Query<&AudioSink, With<WorldMapTheme>>) {
    if let Ok(audio_control) = audio_query.get_single() {
        audio_control.set_volume(0.3);
    }
}

fn volume_in_running(audio_query: Query<&AudioSink, With<WorldMapTheme>>) {
    if let Ok(audio_control) = audio_query.get_single() {
        audio_control.set_volume(0.6);
    }
}

pub fn load_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0,
                window.height() / 2.0,
                0.0,
            ),
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    window.width(),
                    window.height(),
                )),
                ..Default::default()
            },
            texture: asset_server.load("Background/world_map_background.png"),
            ..Default::default()
        },
        WorldMapBackground,
    ));
}

pub fn despawn_background(
    mut commands: Commands,
    world_map_q: Query<Entity, With<WorldMapBackground>>,
) {
    if let Ok(world_map) = world_map_q.get_single() {
        commands.entity(world_map).despawn();
    }
}
