use super::components::*;
use super::enemy::*;
use super::player::*;
use super::resources::*;
use crate::components::Player;
use crate::fight::components::Enemy;
use crate::prelude::*;
use bevy::audio::*;
use bevy::window::PrimaryWindow;

pub struct FightPlugin;

impl Plugin for FightPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<FightState>()
            .add_plugins((PlayerPlugin, EnemyPlugin))
            .add_systems(OnEnter(InGameState::Fight), setup)
            .add_systems(
                Update,
                (
                    intro_timer,
                    flip_sprites.after(intro_timer),
                    move_enemy_sprite,
                    move_player_sprite,
                    intro_timer_check,
                )
                    .run_if(in_state(FightState::Intro))
                    .run_if(in_state(GameState::Running)),
            )
            //Audio handling
            .add_systems(OnEnter(InGameState::Fight), play_fight_music)
            .add_systems(OnExit(InGameState::Fight), stop_fight_music)
            .add_systems(
                OnEnter(GameState::Paused),
                volume_in_pause.run_if(in_state(InGameState::Fight)),
            )
            .add_systems(
                OnEnter(GameState::Running),
                volume_in_running.run_if(in_state(InGameState::Fight)),
            );
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum FightState {
    Intro,
    #[default]
    PlayerTurn,
    EnemyTurn,
    DamageHappening,
}

fn setup(mut commands: Commands, mut next_fight_state: ResMut<NextState<FightState>>) {
    commands.init_resource::<IntroTime>();
    commands.init_resource::<HalfIntroTime>();
    next_fight_state.set(FightState::Intro);
}

pub fn intro_timer(
    mut intro_time: Option<ResMut<IntroTime>>,
    mut half_intro_time: Option<ResMut<HalfIntroTime>>,
    time: Res<Time>,
) {
    if let Some(mut intro_time) = intro_time {
        intro_time.timer.tick(time.delta());
    }
    if let Some(mut half_intro_time) = half_intro_time {
        half_intro_time.timer.tick(time.delta());
    }
    //println!("{:?}", intro_time.timer);
}

pub fn flip_sprites(
    mut commands: Commands,
    mut player_query: Query<&mut Sprite, With<Player>>,
    half_intro_time: Option<Res<HalfIntroTime>>,
) {
    if let Ok(mut player_sprite) = player_query.get_single_mut() {
        if let Some(half_intro_time) = half_intro_time {
            if half_intro_time.timer.finished() {
                commands.remove_resource::<HalfIntroTime>();
                //player_transform.rotation = Quat::from_rotation_y(180.0);
                player_sprite.flip_x = false;
            }
        }
    }
}

fn move_enemy_sprite(
    mut commands: Commands,
    mut enemy_transform_query: Query<(&mut Transform, &Movement), With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    intro_timer: Res<IntroTime>,
    time: Res<Time>,
) {
    let window = window_query.get_single().unwrap();
    let mut enemy_transform = enemy_transform_query.get_single_mut().unwrap();
    let direction = Vec3::new(
        enemy_transform.1.direction.x,
        enemy_transform.1.direction.y,
        0.0,
    );
    enemy_transform.0.translation += direction * enemy_transform.1.speed * time.delta_seconds();
    if intro_timer.timer.finished() {
        commands.remove_resource::<IntroTime>();
    }
}

fn move_player_sprite(
    mut player_transform_query: Query<(&mut Transform, &Movement), With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let window = window_query.get_single().unwrap();
    let mut player_transform = player_transform_query.get_single_mut().unwrap();
    let direction = Vec3::new(
        player_transform.1.direction.x,
        player_transform.1.direction.y,
        0.0,
    );
    player_transform.0.translation += direction * player_transform.1.speed * time.delta_seconds();
}

fn intro_timer_check(
    mut commands: Commands,
    player_query: Query<&Player>,
    enemy_query: Query<&Enemy>,
    intro_timer: Res<IntroTime>,
    mut next_fight_state: ResMut<NextState<FightState>>,
) {
    if intro_timer.timer.finished() {
        commands.remove_resource::<IntroTime>();
        let player = player_query.get_single().unwrap();
        let mut player_starts = false;
        for enemy in enemy_query.iter() {
            if player.speed >= enemy.speed {
                player_starts = true;
            } else {
                player_starts = false;
            }
        }
        if player_starts {
            next_fight_state.set(FightState::PlayerTurn);
            println!("PLAYER TURN")
        } else {
            next_fight_state.set(FightState::EnemyTurn);
            println!("ENEMY TURN")
        }
    }
}

fn play_fight_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fight_theme = asset_server.load("audio/1-17 Battle! [Saturos].wav");
    commands.spawn((
        AudioBundle {
            source: fight_theme,
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                //volume: Volume::Relative(1.0),
                ..Default::default()
            },
        },
        BattleTheme,
    ));
}

fn stop_fight_music(
    mut commands: Commands,
    fight_theme_query: Query<(Entity, &AudioSink), With<BattleTheme>>,
) {
    if let Ok((theme_entity, music_control)) = fight_theme_query.get_single() {
        music_control.stop();
        commands.entity(theme_entity).despawn();
    }
}

fn volume_in_pause(battle_theme_query: Query<&AudioSink, With<BattleTheme>>) {
    if let Ok(battle_theme) = battle_theme_query.get_single() {
        battle_theme.set_volume(0.3);
    }
}

fn volume_in_running(battle_theme_query: Query<&AudioSink, With<BattleTheme>>) {
    if let Ok(battle_theme) = battle_theme_query.get_single() {
        battle_theme.set_volume(1.0);
    }
}
