use crate::prelude::*;
use world_map::*;
use world_map::components::*;
use crate::components::*;
use resources::*;
use bevy::render::camera::ScalingMode;
use bevy::window::PrimaryWindow;

use super::enemy::EnemyPlugin;
use super::player::PlayerPlugin;

pub const ENEMY_SIZE: (f32, f32) = (90.0, 100.0);
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size
pub const NUMBER_OF_ENEMIES: usize = 4;

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins((
            PlayerPlugin,
            EnemyPlugin
        ))
        .add_systems(Update,
            (
                enemy_hit_player,
            )
            .run_if(in_state(AppState::Game))
            .run_if(in_state(GameState::Running))
            .run_if(in_state(InGameState::WorldMap))
        )
        .add_systems(Update,
            (
                timer_after_collission_check,
                global_timer_update
            )
            .run_if(in_state(InGameState::WorldMap))
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
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut next_game_state: ResMut<NextState<GameState>>
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE.0 / 2.0;

            if distance < player_radius + enemy_radius {
                println!("Collision!, ATTACK");
                commands.insert_resource(PlayerEntity{
                    entity: player_entity
                });
                commands.insert_resource(EnemyEntity{
                    entity: enemy_entity
                });
                commands.init_resource::<AfterEnemyCollisionTimer>();
                next_game_state.set(GameState::Paused);
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
        println!("{:?}", time_value);
    }
}

pub fn timer_after_collission_check(
    mut commands: Commands,
    collision_timer: Option<Res<AfterEnemyCollisionTimer>>,
    mut next_state: ResMut<NextState<InGameState>>,
    mut next_game_state: ResMut<NextState<GameState>>
) {
    if let Some(timer) = &collision_timer {
        if timer.timer.finished() {
            commands.remove_resource::<AfterEnemyCollisionTimer>();
            println!("FIGHT STATE");
            next_state.set(InGameState::Fight);
            next_game_state.set(GameState::Running);
        }
    }
}

pub fn fight(mut characters: Query<(&mut Player, &mut Enemy)>, input: Res<Input<KeyCode>>) {
    println!("{:?}", characters);
    for (mut player, mut enemy) in &mut characters {
        //println!("{}, {}", player., enemy );
        /*if input.pressed(KeyCode::E) {
            if rand::random() {
                enemy.health -= player.damage;
            } else {
                player.health -= enemy.damage;
            }
        }*/

        //println!("Player health: {}, Enemy health: {}", player.health, enemy.health);
    }
}