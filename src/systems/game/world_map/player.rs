use crate::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::Game), spawn_player)
        .add_systems(Update,
            (
                character_movement.before(confine_player_movement),
                confine_player_movement
            )
            .run_if(in_state(AppState::Game))
            .run_if(in_state(GameState::Running))
            .run_if(in_state(InGameState::WorldMap))
        )
        .add_systems(OnExit(AppState::Game), despawn_player)
        .add_systems(OnExit(InGameState::WorldMap), despawn_player);
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let texture: Handle<Image> = asset_server.load("sara_stand.png");
    //player
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            sprite: Sprite {
                //custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            texture,
            ..default()
        },
        Player {
            damage: 10,
            mov_speed: 500.0,
            speed: 100,
            ..Default::default()
        },
    ));
}


fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>
) {
    let player_entity = player_query.get_single().unwrap();
    commands.entity(player_entity).despawn();
}

pub fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.mov_speed * time.delta_seconds();

        if input.pressed(KeyCode::W) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += movement_amount;
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= movement_amount;
        }
        //println!("{:?}", transform);
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0; //32
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        //Bound the player in x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        //Bound the player in y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}