use crate::prelude::fight::components::Enemy;
use crate::prelude::fight::in_fight::FightState;
use crate::systems::game::fight::turns::events::*;
use crate::systems::game::fight::turns::resources::*;
use crate::systems::game::resources::PlayerStatus;
use bevy::prelude::*;
use rand::prelude::*;

pub fn player_attack(
    player_status: &Res<PlayerStatus>,
    event_writter: &mut EventWriter<PlayerDamageEvent>,
) {
    let attack_is_crit = thread_rng().gen_bool(0.5);
    let damage_amount = player_status.damage;
    if attack_is_crit {
        println!("CRIT!");
        event_writter.send(PlayerDamageEvent(damage_amount * 2.));
    } else {
        event_writter.send(PlayerDamageEvent(damage_amount));
    }
}

pub fn player_does_damage_check(
    mut enemy_query: Query<&mut Enemy, With<Enemy>>,
    mut event_reader: EventReader<PlayerDamageEvent>,
) {
    for event in event_reader.iter() {
        for mut enemy in enemy_query.iter_mut() {
            println!("Player does {} damage to the enemy!", event.0);
            enemy.health -= event.0;
        }
    }
}

pub fn damage_happening_timer(mut commands: Commands) {
    commands.init_resource::<DamageHappeningTimer>();
}

pub fn damage_happening_ticker(time: Res<Time>, mut timer: Option<ResMut<DamageHappeningTimer>>) {
    if let Some(mut timer) = timer {
        timer.timer.tick(time.delta());
        //println!("{:?}", timer.timer);
    }
}

pub fn damage_happening_timer_check(
    mut commands: Commands,
    damage_timer: Option<Res<DamageHappeningTimer>>,
    mut next_fight_state: ResMut<NextState<FightState>>,
    player_active_last_turn: Res<PlayerActiveLastTurn>,
) {
    let player_was_active_last_turn = player_active_last_turn.0;
    if let Some(damage_timer) = damage_timer {
        if damage_timer.timer.finished() {
            commands.remove_resource::<DamageHappeningTimer>();
            if player_was_active_last_turn {
                next_fight_state.set(FightState::EnemyTurn);
                println!("ENEMY TURN")
            } else {
                next_fight_state.set(FightState::PlayerTurn);
                println!("PLAYER TURN")
            }
        }
    }
}

pub fn enemy_turn(
    mut next_fight_state: ResMut<NextState<FightState>>,
    mut event_writter: EventWriter<EnemyDamageEvent>,
    mut player_active_last_turn: ResMut<PlayerActiveLastTurn>,
) {
    println!("ENEMY DOES SOMETHING!");
    player_active_last_turn.0 = false;
    next_fight_state.set(FightState::DamageHappening);
    event_writter.send(EnemyDamageEvent(20.0));
}

pub fn enemy_does_damage_check(
    mut player_status: ResMut<PlayerStatus>,
    mut event_reader: EventReader<EnemyDamageEvent>,
    player_defending: Res<PlayerIsDefending>,
) {
    for event in event_reader.iter() {
        let player_is_defending = player_defending.0;
        let mut enemy_damage = event.0;
        if player_is_defending {
            enemy_damage -= enemy_damage * 0.25;
            println!("Enemy does {} damage to the player", enemy_damage);
            player_status.health -= enemy_damage;
        } else {
            println!("Enemy does {} damage to the player", enemy_damage);
            player_status.health -= enemy_damage;
        }
    }
}
