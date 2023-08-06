use super::components::*;
use super::enemy::*;
use super::player::*;
use super::resources::*;
use super::turns::TurnsPlugin;
use super::turns::events::CombatLogEvent;
use super::turns::systems::clear_combat_log;
use crate::components::Player;
use crate::fight::components::Enemy;
use crate::prelude::*;
use crate::systems::game::fight::turns::resources::*;
use bevy::audio::*;
use bevy::transform;
use bevy::window::PrimaryWindow;

pub struct FightPlugin;

impl Plugin for FightPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<FightState>()
            .add_event::<CombatLogEvent>()
            .init_resource::<PlayerActiveLastTurn>()
            .add_plugins((PlayerPlugin, EnemyPlugin, TurnsPlugin))
            .add_systems(OnEnter(InGameState::Fight), setup)
            // intro
            .add_systems(
                Update,
                (
                    intro_timer,
                    flip_sprites.after(intro_timer),
                    move_enemy_sprite,
                    move_player_sprite,
                    intro_timer_check,
                    move_background,
                )
                    .run_if(in_state(FightState::Intro))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(Update, handle_combat_log_events.run_if(in_state(InGameState::Fight)))
            .add_systems(OnExit(InGameState::Fight), (despawn_background, clear_combat_log))
            //Audio handling
            .add_systems(OnEnter(InGameState::Fight), (play_fight_music))
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
    Win,
    Lost
}

fn setup(
    mut commands: Commands,
    mut next_fight_state: ResMut<NextState<FightState>>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    commands.init_resource::<IntroTime>();
    commands.init_resource::<HalfIntroTime>();
    load_background(&mut commands, asset_server, window_query);
    next_fight_state.set(FightState::Intro);
}

pub fn intro_timer(
    mut intro_time: Option<ResMut<IntroTime>>,
    mut half_intro_time: Option<ResMut<HalfIntroTime>>,
    time: Res<Time>,
) {
    if let Some(mut intro_time) = intro_time {
        //println!("{:?}", intro_time.timer);
        intro_time.timer.tick(time.delta());
    }
    if let Some(mut half_intro_time) = half_intro_time {
        half_intro_time.timer.tick(time.delta());
    }
    //println!("{:?}", intro_time.timer);
}

pub fn flip_sprites(
    mut commands: Commands,
    mut player_query: Query<&mut Sprite, With<FightPlayer>>,
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
    mut enemy_transform_query: Query<(&mut Transform, &Movement), With<FightEnemy>>,
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
    mut player_transform_query: Query<(&mut Transform, &Movement), With<FightPlayer>>,
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
    player_query: Query<&Player, With<FightPlayer>>,
    enemy_query: Query<&Enemy, With<FightEnemy>>,
    intro_timer: Res<IntroTime>,
    mut next_fight_state: ResMut<NextState<FightState>>,
    mut player_active_last_turn: ResMut<PlayerActiveLastTurn>,
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
            player_active_last_turn.0 = true;
            println!("PLAYER TURN")
        } else {
            next_fight_state.set(FightState::EnemyTurn);
            player_active_last_turn.0 = false;
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
                volume: Volume::Relative(VolumeLevel::new(0.6)),
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
        battle_theme.set_volume(0.6);
    }
}

fn load_background(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                0.0 + (window.width() / 4.0),
                window.height() / 2.0,
                0.0,
            ),
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    window.width() + (window.width() / 2.0),
                    window.height(),
                )),
                flip_x: true,
                ..default()
            },
            texture: asset_server.load("Background/mathieu-chauderlot-room-0023-layer-5.png"),
            ..Default::default()
        },
        FightBackGround,
        Movement {
            direction: Vec2::new(1.0, 0.0).normalize(),
            speed: 500.0,
        },
    ));

    /*commands.spawn(ImageBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            //margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
            ..Default::default()
        },
        image: asset_server
            .load("Background/mathieu-chauderlot-room-0023-layer-5.png")
            .into(),
        ..Default::default()
    });*/
}

fn move_background(
    mut background_query: Query<(&mut Transform, &Movement), With<FightBackGround>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, movement)) = background_query.get_single_mut() {
        let direction = Vec3::new(movement.direction.x, movement.direction.y, 0.0);
        transform.translation += direction * movement.speed * time.delta_seconds();
    }
}

fn despawn_background(
    mut commands: Commands,
    background_query: Query<Entity, With<FightBackGround>>,
) {
    if let Ok(entity) = background_query.get_single() {
        commands.entity(entity).despawn();
    }
}

pub fn handle_combat_log_events(
    mut combat_log_event: EventReader<CombatLogEvent>,
    mut combat_log: ResMut<CombatLog>
) {
    for combat_log_event in combat_log_event.iter() {
        combat_log.logs.insert(0, (combat_log_event.log.clone(), combat_log_event.color));
    }
    //combat_log.logs.sort_by_key(|b|  )
}