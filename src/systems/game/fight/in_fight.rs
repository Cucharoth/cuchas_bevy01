use bevy::window::PrimaryWindow;
use crate::fight::components::Enemy;
use super::enemy::*;
use super::player::*;
use super::resources::*;
use super::components::*;
use crate::components::Player;
use crate::prelude::*;

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
                    intro_timer_check
                )
                .run_if(in_state(FightState::Intro))
                .run_if(in_state(GameState::Running))
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
                println!("FLIP");
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
    time: Res<Time>
) {
    let window = window_query.get_single().unwrap();
    let mut enemy_transform = enemy_transform_query.get_single_mut().unwrap();
    let direction = Vec3::new(enemy_transform.1.direction.x, enemy_transform.1.direction.y, 0.0);
    enemy_transform.0.translation += direction * enemy_transform.1.speed * time.delta_seconds();
    if intro_timer.timer.finished() {
        commands.remove_resource::<IntroTime>();

    }
}

fn move_player_sprite(
    mut player_transform_query: Query<(&mut Transform, &Movement), With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>
) {
    let window = window_query.get_single().unwrap();
    let mut player_transform = player_transform_query.get_single_mut().unwrap();
    let direction = Vec3::new(player_transform.1.direction.x, player_transform.1.direction.y, 0.0);
    player_transform.0.translation += direction * player_transform.1.speed * time.delta_seconds();
}

fn intro_timer_check(
    mut commands: Commands,
    player_query: Query<&Player>,
    enemy_query: Query<&Enemy>,
    intro_timer: Res<IntroTime>,
    mut next_fight_state: ResMut<NextState<FightState>>
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
        } else {
            next_fight_state.set(FightState::EnemyTurn);
        }
    }
}
