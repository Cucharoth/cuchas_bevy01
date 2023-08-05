use crate::prelude::fight::components::*;
use crate::prelude::fight::in_fight::FightState;
use crate::systems::game::fight::turns::events::*;
use crate::systems::game::fight::turns::resources::*;
use crate::systems::game::resources::PlayerStatus;
use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde::__private::de;
use rand::prelude::*;

pub fn player_attack(
    player_status: &Res<PlayerStatus>,
    event_writter: &mut EventWriter<PlayerDamageEvent>,
) {
    let attack_is_crit = thread_rng().gen_bool(0.5);
    let damage_amount = player_status.damage;
    if attack_is_crit {
        println!("CRIT!");
        event_writter.send(PlayerDamageEvent {
            damage: damage_amount * 2.,
            debuff: None,
            debuff_duration: None
        })
    } else {
        event_writter.send(PlayerDamageEvent {
            damage: damage_amount,
            debuff: None,
            debuff_duration: None
        });
    }
}

pub fn player_does_damage_check(
    mut enemy_query: Query<&mut Enemy, With<Enemy>>,
    mut event_reader: EventReader<PlayerDamageEvent>,
) {
    for event in event_reader.iter() {
        for mut enemy in enemy_query.iter_mut() {
            let player_damage = event.damage;
            println!("Player does {:?} damage to the enemy!", player_damage);
            enemy.health -= player_damage;
            if let Some(debuff) = event.debuff.clone() {
                match debuff {
                    Debuff::Freezing => {
                        let is_applied = thread_rng().gen_bool(0.6);
                        if is_applied {
                            println!("Player applies {:?}", debuff);
                            enemy.debuffs.insert(debuff, (event.debuff_duration.unwrap(), player_damage));
                        } else {
                            println!("Enemy resisted the Freezing!")
                        }
                    }
                    Debuff::Burning => {
                        println!("Player applies {:?}", debuff);
                        enemy.debuffs.insert(debuff, (event.debuff_duration.unwrap(), player_damage));
                    }
                    Debuff::Blindness => { enemy.debuffs.insert(debuff, (event.debuff_duration.unwrap(), player_damage)); }
                }
            }
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
    mut enemy_q: Query<&mut Enemy, With<FightEnemy>>,
    mut next_fight_state: ResMut<NextState<FightState>>,
    mut event_writter: EventWriter<EnemyDamageEvent>,
    mut player_active_last_turn: ResMut<PlayerActiveLastTurn>,
) { 
    if let Ok(mut enemy) = enemy_q.get_single_mut() {
        let mut enemy_is_frozen = false;
        let mut enemy_is_blind = false;

        let enemy_debuff_clone = enemy.debuffs.clone();
        for (debuff, (duration, player_damage)) in enemy_debuff_clone.iter() {
            //debuff handle
            match debuff {
                Debuff::Freezing => { enemy_is_frozen = true;},
                Debuff::Burning => { enemy.health -= player_damage * 0.20},
                Debuff::Blindness => { enemy_is_blind = true;},

            }
            if *duration <= 1. {
                enemy.debuffs.remove(debuff);
            } else {
                if let Some((enemy_debuff_duration, _player_damage)) = enemy.debuffs.get_mut(debuff) {
                    *enemy_debuff_duration = -1.;
                }
            }
        }

        println!("ENEMY DOES SOMETHING!");
        player_active_last_turn.0 = false;
        let enemy_damage = 20.;
        // frozen
        if enemy_is_frozen {
            println!("Enemy is Frozen!");       
            next_fight_state.set(FightState::DamageHappening);
        } else if enemy_is_blind {
            println!("Enemy CAN'T C");
            next_fight_state.set(FightState::DamageHappening);
        } else {
            event_writter.send(EnemyDamageEvent(enemy_damage));
            next_fight_state.set(FightState::DamageHappening);
        }
    }

    
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
