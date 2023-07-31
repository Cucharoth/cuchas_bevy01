use rand::random;
use bevy::window::PrimaryWindow;
use crate::prelude::*;
use world_map::components::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::Game), spawn_enemy)
        .add_systems(Update,
            (
                update_enemy_direction,
                confine_enemy_movement,
                enemy_movement
            )
            .run_if(in_state(AppState::Game))
            .run_if(in_state(GameState::Running)),
        )
        .add_systems(OnExit(AppState::Game), despawn_enemies)
        .add_systems(OnExit(InGameState::WorldMap), despawn_enemies);
    }
}

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("Enemy/devil.png"),
                ..default()
            },
            Enemy {
                damage: 10,
                speed: 20.0,
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                spawn: (random_x, random_y),
            },
        ));
    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>
) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * enemy.speed * time.delta_seconds();
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_x_enemy_size = ENEMY_SIZE.0 / 2.0;
    let half_y_enemy_size = ENEMY_SIZE.1 / 2.0;

    let x_min = 0.0 + half_x_enemy_size;
    let x_max = window.width() - half_x_enemy_size;
    let y_min = 0.0 + half_y_enemy_size;
    let y_max = window.height() - half_y_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
        }
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
) {
    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;

        let x_min = enemy.spawn.0 - 50.0;
        let x_max = enemy.spawn.0 + 50.0;
        let y_min = enemy.spawn.1 - 50.0;
        let y_max = enemy.spawn.1 + 50.0;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
        }
    }
}