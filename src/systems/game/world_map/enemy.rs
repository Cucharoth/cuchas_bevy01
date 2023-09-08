use crate::prelude::*;
use crate::prelude::fight::player::spawn_player;
use crate::systems::game::resources::*;
use bevy::window::PrimaryWindow;
use rand::random;
use world_map::components::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // from menu
            .add_systems(
                OnTransition {
                    from: InGameState::StandBy,
                    to: InGameState::WorldMap,
                },
                spawn_enemy.after(spawn_player),
            )
            // from fight
            .add_systems(
                OnTransition {
                    from: InGameState::Fight,
                    to: InGameState::WorldMap,
                },
                restore_enemy_visibility,
            )
            // while on map
            .add_systems(
                Update,
                (
                    update_enemy_direction.after(enemy_movement),
                    confine_enemy_movement.after(update_enemy_direction),
                    enemy_movement,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Running))
                    .run_if(in_state(InGameState::WorldMap))
            )
            // on exit map
            .add_systems(OnExit(AppState::Game), despawn_enemies)
            .add_systems(OnExit(InGameState::WorldMap), (hide_enemies))
            .add_systems(
                OnTransition {
                    from: InGameState::WorldMap,
                    to: InGameState::Fight,
                },
                despawn_enemy_we_hit,
            );
    }
}

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    player_status: Res<PlayerStatus>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let mut enemy_transform: Transform;
        let mut random_x: f32;
        let mut random_y: f32;

        loop {
            random_x = random::<f32>() * window.width();
            random_y = random::<f32>() * window.height();
            enemy_transform = Transform::from_xyz(random_x, random_y, 900.0);

            let distance = player_status
                .transform
                .translation
                .distance(enemy_transform.translation);
            let safe_distance = 200.0;
            if distance > safe_distance {
                break;
            }
        }

        commands.spawn((
            SpriteBundle {
                transform: enemy_transform,
                texture: asset_server.load("Enemy/devil.png"),
                ..default()
            },
            Enemy {
                damage: 10,
                speed: 20.0,
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                spawn: (random_x, random_y),
            },
            WorldMapEnemy,
        ));
    }
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
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
            enemy.direction.x *= -1.;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.;
        }
    }
}

pub fn update_enemy_direction(mut enemy_query: Query<(&Transform, &mut Enemy)>) {
    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        let max_enemy_distance = 100.;
        let x_min = enemy.spawn.0 - max_enemy_distance;
        let x_max = enemy.spawn.0 + max_enemy_distance;
        let y_min = enemy.spawn.1 - max_enemy_distance;
        let y_max = enemy.spawn.1 + max_enemy_distance;
        if translation.x < x_min || translation.x > x_max {
            let rdm = random::<f32>();
            if rdm > 0.5 {
                enemy.direction.y *= -1.;
            }
            enemy.direction.x *= -1.0;
        }
        if translation.y < y_min || translation.y > y_max {
            let rdm = random::<f32>();
            if rdm > 0.5 {
                enemy.direction.x *= -1.;
            }
            enemy.direction.y *= -1.0;
        }
    }
}

fn hide_enemies(mut enemy_query: Query<&mut Visibility, With<WorldMapEnemy>>) {
    for mut enemy_visibility in enemy_query.iter_mut() {
        *enemy_visibility = Visibility::Hidden;
    }
}

fn restore_enemy_visibility(mut enemy_query: Query<&mut Visibility, With<WorldMapEnemy>>) {
    for mut enemy_visibility in enemy_query.iter_mut() {
        *enemy_visibility = Visibility::Visible;
    }
}

fn despawn_enemy_we_hit(mut commands: Commands, enemy_we_hit: Res<EnemyEntityCollisioned>) {
    commands.entity(enemy_we_hit.entity).despawn();
}

pub fn spawn_single_enemy(
    commands: &mut Commands,
    window_query: &Query<&Window, With<PrimaryWindow>>,
    asset_server: &Res<AssetServer>,
    player_status: &ResMut<PlayerStatus>,
) {
    let window = window_query.get_single().unwrap();
    let mut enemy_transform: Transform;
    let mut random_x: f32;
    let mut random_y: f32;

    loop {
        random_x = random::<f32>() * window.width();
        random_y = random::<f32>() * window.height();
        enemy_transform = Transform::from_xyz(random_x, random_y, 900.0);

        let distance = player_status
            .transform
            .translation
            .distance(enemy_transform.translation);
        // adjusted radius so it spawns further
        let safe_distance = 200.;
        if distance > safe_distance {
            break;
        }
    }

    commands.spawn((
        SpriteBundle {
            transform: enemy_transform,
            texture: asset_server.load("Enemy/devil.png"),
            ..default()
        },
        Enemy {
            damage: 10,
            speed: 20.0,
            direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            spawn: (random_x, random_y),
        },
        WorldMapEnemy,
    ));
}
