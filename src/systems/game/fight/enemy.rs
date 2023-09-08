use super::components::*;
use crate::prelude::*;
use bevy::{window::PrimaryWindow, utils::HashMap};
use rand::{random, thread_rng, Rng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::Fight), spawn_enemy)
            .add_systems(OnExit(AppState::Game), despawn_enemies)
            .add_systems(OnExit(InGameState::Fight), despawn_enemies);
    }
}

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let enemy_speed = thread_rng().gen_range(91..102) as f32; // [min, max)
    let enemy = Enemy {
        health: 100.,
        damage: 10.,
        speed: enemy_speed,
        debuffs: HashMap::new()
    };
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() - window.width() / 8.0,
                window.height() - window.height() / 2.0,
                800.0,
            ),
            sprite: Sprite {
                custom_size: Some(Vec2::new(300.0, 300.0)),
                ..default()
            },
            texture: asset_server.load("Enemy/devil.png"),
            ..default()
        },
        enemy,
        Movement {
            direction: Vec2::new(-1.0, 0.0).normalize(),
            speed: 900.0,
        },
        FightEnemy
    ));
}

fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}
