use super::components::Movement;
use super::{components::FightPlayer, resources::*};
use crate::prelude::fight::components::*;
use crate::prelude::{components::Player, *};
use crate::systems::ui::fight::resources::SaraSprites;
use bevy::window::PrimaryWindow;
use resources::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::Fight), spawn_player)
            .add_systems(OnExit(InGameState::Fight), (despawn_player, despawn_extra_sprites));
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_status: Res<PlayerStatus>,
    asset_server: Res<AssetServer>,
    sara_sprites: Res<SaraSprites>,
) {
    println!("creating player in fight");
    let window = window_query.get_single().unwrap();
    let texture = asset_server.load("sara_stand.png");

    spawn_extra_sprites(&mut commands, sara_sprites);

    commands.insert_resource(PlayerSprite {
        sprite: texture.to_owned(),
    });

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                0.0 + (window.width() / 10.0),
                window.height() / 7.0,
                900.0,
            ),
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
        Movement {
            direction: Vec2::new(1.0, 0.0).normalize(),
            speed: 900.0,
        },
        FightPlayer,
    ));
    println!("{:?}", player_status);
}

fn despawn_player(mut commands: Commands, fight_player_q: Query<Entity, With<FightPlayer>>) {
    let fight_player_entity = fight_player_q.get_single().unwrap();
    commands.entity(fight_player_entity).despawn();
}

pub fn change_player_sprite(
    mut commands: Commands,
    mut player_query: Query<&mut Sprite, With<Player>>,
) {
    for sprite in player_query.iter_mut() {}
}

fn spawn_extra_sprites(commands: &mut Commands, sara_sprites: Res<SaraSprites>) {
    commands.spawn((
        SpriteBundle {
            texture: sara_sprites.sprites[0].clone(),
            transform: Transform {
                translation: Vec3 {
                    x: 1038.8475,
                    y: 102.85714,
                    z: 900.0,
                },
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(300.0, 300.0)),
                flip_x: false,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        SaraCast,
        ExtraSprite
    ));
    commands.spawn((
        SpriteBundle {
            texture: sara_sprites.sprites[1].clone(),
            transform: Transform {
                translation: Vec3 {
                    x: 1038.8475,
                    y: 102.85714,
                    z: 900.0,
                },
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(300.0, 300.0)),
                flip_x: false,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        SaraCastSuccesful,
        ExtraSprite
    ));
    commands.spawn((
        SpriteBundle {
            texture: sara_sprites.sprites[2].clone(),
            transform: Transform {
                translation: Vec3 {
                    x: 1100.8475,
                    y: 102.85714,
                    z: 900.0,
                },
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(400.0, 300.0)),
                flip_x: false,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        SaraDedge,
        ExtraSprite
    ));
}

fn despawn_extra_sprites(
    mut commands: Commands,
    extra_sprites_q: Query<Entity, With<ExtraSprite>>
) {
    for sprite in extra_sprites_q.iter() {
        commands.entity(sprite).despawn();
    }
}