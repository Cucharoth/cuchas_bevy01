use super::resources::*;
use super::components::Movement;
use crate::prelude::{components::Player, *};
use bevy::window::PrimaryWindow;
use resources::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::Fight), spawn_player);
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_status: Res<PlayerStatus>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let texture = asset_server.load("sara_stand.png");
    
    commands.insert_resource(
        PlayerSprite{
            sprite: texture.to_owned()
        }
    );

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0 + (window.width() / 10.0), window.height() / 7.0, 900.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(300.0, 300.0)),
                flip_x: true,
                ..default()
            },
            texture,
            ..default()
        },
        Player {
            health: player_status.health,
            damage: player_status.damage,
            speed: player_status.speed,
            ..Default::default()
        },
        Movement{ 
            direction: Vec2::new(1.0, 0.0).normalize(),
            speed: 900.0
        }
    ));
    println!("{:?}", player_status);
}

pub fn change_player_sprite(
    mut commands: Commands,
    mut player_query: Query<&mut Sprite, With<Player>>
) {
    for sprite in player_query.iter_mut() {
        
    }
}
