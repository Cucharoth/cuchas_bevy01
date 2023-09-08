use crate::AppState;
use crate::prelude::InGameState;
use crate::prelude::fight::components::*;
use crate::prelude::fight::in_fight::FightState;
use crate::systems::game::fight::turns::events::*;
use crate::systems::game::fight::turns::resources::*;
use crate::systems::game::resources::PlayerStatus;
use crate::systems::ui::fight::systems::style::*;
use bevy::prelude::*;
use rand::prelude::*;

pub fn player_attack(
    player_status: &Res<PlayerStatus>,
    event_writter: &mut EventWriter<PlayerDamageEvent>,
    combat_log_event_writer: &mut EventWriter<CombatLogEvent>
) {
    let attack_is_crit = thread_rng().gen_bool(0.5);
    let damage_amount = player_status.damage;
    if attack_is_crit {
        combat_log_event_writer.send(
            CombatLogEvent {
                log: "CRIT!".to_string(),
                color: FIGHT_COMBAT_LOG_TEXT_COLOR
            }
        );
        event_writter.send(PlayerDamageEvent {
            damage: damage_amount * 2.,
            debuff: None,
            debuff_duration: None,
        })
    } else {
        event_writter.send(PlayerDamageEvent {
            damage: damage_amount,
            debuff: None,
            debuff_duration: None,
        });
    }
}

pub fn player_does_damage_check(
    mut enemy_query: Query<&mut Enemy, With<Enemy>>,
    mut event_reader: EventReader<PlayerDamageEvent>,
    mut combat_log_event_writer: EventWriter<CombatLogEvent>
) {
    for event in event_reader.iter() {
        for mut enemy in enemy_query.iter_mut() {
            let player_damage = event.damage;
            combat_log_event_writer.send(
                CombatLogEvent {
                    log: format!("Player does {:?} damage to the enemy!", player_damage),
                    color: FIGHT_COMBAT_LOG_GOOD_TEXT_COLOR
                }
            );
            enemy.health -= player_damage;
            if let Some(debuff) = event.debuff.clone() {
                match debuff {
                    Debuff::Freezing => {
                        let is_applied = thread_rng().gen_bool(0.6);
                        if is_applied {
                            combat_log_event_writer.send(
                                CombatLogEvent {
                                    log: format!("Player applies {:?}", debuff),
                                    color: FIGHT_COMBAT_LOG_TEXT_COLOR
                                }
                            );
                            enemy
                                .debuffs
                                .insert(debuff, (event.debuff_duration.unwrap(), player_damage));
                        } else {
                            combat_log_event_writer.send(
                                CombatLogEvent {
                                    log: format!("Enemy resisted the Freezing!"),
                                    color: FIGHT_COMBAT_LOG_TEXT_COLOR
                                }
                            );
                        }
                    }
                    Debuff::Burning => {
                        combat_log_event_writer.send(
                            CombatLogEvent {
                                log: format!("Player applies {:?}", debuff),
                                color: FIGHT_COMBAT_LOG_TEXT_COLOR
                            }
                        );
                        enemy
                            .debuffs
                            .insert(debuff, (event.debuff_duration.unwrap(), player_damage));
                    }
                    Debuff::Blindness => {
                        enemy
                            .debuffs
                            .insert(debuff, (event.debuff_duration.unwrap(), player_damage));
                    }
                }
            }
        }
    }
}

pub fn damage_happening_timer(mut commands: Commands) {
    commands.init_resource::<DamageHappeningTimer>();
}

pub fn fight_win_timer(mut commands: Commands) {
    commands.init_resource::<FightWinTimer>();
}

pub fn fight_lost_timer(mut commands: Commands) {
    commands.init_resource::<FightLostTimer>();
}

pub fn damage_happening_ticker(time: Res<Time>, timer: Option<ResMut<DamageHappeningTimer>>) {
    if let Some(mut timer) = timer {
        timer.timer.tick(time.delta());
        //println!("{:?}", timer.timer);
    }
}

pub fn fight_win_ticker(time: Res<Time>, timer: Option<ResMut<FightWinTimer>>) {
    if let Some(mut timer) = timer {
        timer.timer.tick(time.delta());
        //println!("{:?}", timer.timer);
    }
}

pub fn fight_lost_ticker(time: Res<Time>, timer: Option<ResMut<FightLostTimer>>) {
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
    mut combat_log_event_writer: EventWriter<CombatLogEvent>
) {
    let player_was_active_last_turn = player_active_last_turn.0;
    if let Some(damage_timer) = damage_timer {
        if damage_timer.timer.finished() {
            commands.remove_resource::<DamageHappeningTimer>();
            if player_was_active_last_turn {
                next_fight_state.set(FightState::EnemyTurn);
                combat_log_event_writer.send(
                    CombatLogEvent {
                        log: "ENEMY TURN".to_string(),
                        color: FIGHT_COMBAT_LOG_EMPHASIS_TEXT_COLOR
                    }
                );
            } else {
                next_fight_state.set(FightState::PlayerTurn);
                combat_log_event_writer.send(
                    CombatLogEvent {
                        log: "PLAYER TURN".to_string(),
                        color: FIGHT_COMBAT_LOG_EMPHASIS_TEXT_COLOR
                    }
                );
            }
        }
    }
}

pub fn enemy_turn(
    mut enemy_q: Query<&mut Enemy, With<FightEnemy>>,
    mut next_fight_state: ResMut<NextState<FightState>>,
    mut event_writter: EventWriter<EnemyDamageEvent>,
    mut player_active_last_turn: ResMut<PlayerActiveLastTurn>,
    mut combat_log_event_writer: EventWriter<CombatLogEvent>
) {
    if let Ok(mut enemy) = enemy_q.get_single_mut() {
        let mut enemy_is_frozen = false;
        let mut enemy_is_blind = false;

        let enemy_debuff_clone = enemy.debuffs.clone();
        for (debuff, (duration, player_damage)) in enemy_debuff_clone.iter() {
            //debuff handle
            match debuff {
                Debuff::Freezing => {
                    enemy_is_frozen = true;
                }
                Debuff::Burning => {
                    let burning_amount = player_damage * 0.20;
                    combat_log_event_writer.send(
                        CombatLogEvent {
                            log: format!("Enemy is burning!, suffers {} damage", burning_amount),
                            color: FIGHT_COMBAT_LOG_GOOD_TEXT_COLOR
                        }
                    );
                    enemy.health -= burning_amount
                }
                Debuff::Blindness => {
                    enemy_is_blind = true;
                }
            }
            if *duration <= 1. {
                enemy.debuffs.remove(debuff);
            } else {
                if let Some((enemy_debuff_duration, _player_damage)) = enemy.debuffs.get_mut(debuff)
                {
                    *enemy_debuff_duration = -1.;
                }
            }
        }

        combat_log_event_writer.send(
            CombatLogEvent {
                log: format!("ENEMY DOES SOMETHING!"),
                color: FIGHT_COMBAT_LOG_TEXT_COLOR
            }
        );
        player_active_last_turn.0 = false;
        let enemy_damage = 20.;
        // frozen
        if enemy_is_frozen {
            combat_log_event_writer.send(
                CombatLogEvent {
                    log: format!("Enemy is Frozen!"),
                    color: FIGHT_COMBAT_LOG_TEXT_COLOR
                }
            );
            next_fight_state.set(FightState::DamageHappening);
        } else if enemy_is_blind {
            combat_log_event_writer.send(
                CombatLogEvent {
                    log: format!("Enemy CAN'T C"),
                    color: FIGHT_COMBAT_LOG_TEXT_COLOR
                }
            );
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
    mut combat_log_event_writer: EventWriter<CombatLogEvent>
) {
    for event in event_reader.iter() {
        let player_is_defending = player_defending.0;
        let mut enemy_damage = event.0;
        if player_is_defending {
            enemy_damage -= enemy_damage * 0.25;
            combat_log_event_writer.send(
                CombatLogEvent {
                    log: format!("Enemy does {} damage to the player", enemy_damage),
                    color: FIGHT_COMBAT_LOG_BAD_TEXT_COLOR
                }
            );
            player_status.health -= enemy_damage;
        } else {
            combat_log_event_writer.send(
                CombatLogEvent {
                    log: format!("Enemy does {} damage to the player", enemy_damage),
                    color: FIGHT_COMBAT_LOG_BAD_TEXT_COLOR
                }
            );
            player_status.health -= enemy_damage;
        }
    }
}

pub fn check_if_enemy_is_dead(
    mut enemy_q: Query<(&Enemy, &mut Visibility), With<FightEnemy>>,
    mut next_fight_state: ResMut<NextState<FightState>>,
    mut combat_log_event_writer: EventWriter<CombatLogEvent>
) {
    let (enemy, mut enemy_visibility) = enemy_q.get_single_mut().unwrap();
    if enemy.health <= 0. {
        *enemy_visibility = Visibility::Hidden;
        combat_log_event_writer.send(
            CombatLogEvent {
                log: format!("Enemy is dead!"),
                color: FIGHT_COMBAT_LOG_TEXT_COLOR
            }
        );
        next_fight_state.set(FightState::Win);
    }
}

pub fn win_timer_check(
    mut commands: Commands,
    damage_timer: Option<Res<FightWinTimer>>,
    mut next_ingame_state: ResMut<NextState<InGameState>>,
    mut player_status: ResMut<PlayerStatus>
) {
    if let Some(damage_timer) = damage_timer {
        if damage_timer.timer.finished() {
            commands.remove_resource::<FightWinTimer>();
            let bonus_hp = 50.;
            let bonus_mp = 50.;
            //println!("{}", player_status.health + bonus_hp );
            player_status.health = if (player_status.health + bonus_hp) < 100. {
                player_status.health + bonus_hp 
            } else {
                100.
            };

            player_status.mana = if (player_status.mana + bonus_mp) < 100. {
                player_status.mana + bonus_mp 
            } else {
                100.
            };
            
            next_ingame_state.set(InGameState::WorldMap);
        }
    }
}

pub fn check_if_player_is_dead(
    mut sara_dedge_q: Query<&mut Visibility, (With<SaraDedge>, Without<FightPlayer>)>,
    mut player_sprite_q: Query<&mut Visibility, (With<FightPlayer>, Without<SaraDedge>)>,
    mut next_app_state: ResMut<NextState<FightState>>,
    player_status: ResMut<PlayerStatus>,
    mut combat_log_event_writer: EventWriter<CombatLogEvent>
) {
    if player_status.health <= 0. {
        combat_log_event_writer.send(
            CombatLogEvent {
                log: format!("Player is dead.. GG."),
                color: FIGHT_COMBAT_LOG_BAD_TEXT_COLOR
            }
        );
        if let Ok(mut player_visibility) = player_sprite_q.get_single_mut() {
            *player_visibility = Visibility::Hidden; 
        }
        let mut sara_dedge = sara_dedge_q.get_single_mut().unwrap();
        *sara_dedge = Visibility::Visible;
        next_app_state.set(FightState::Lost)
    }
}

pub fn fight_lost_timer_check(
    mut commands: Commands,
    damage_timer: Option<Res<FightLostTimer>>,
    mut player_status: ResMut<PlayerStatus>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_ingame_state: ResMut<NextState<InGameState>>,
    mut next_fight_state: ResMut<NextState<FightState>>
) {
    if let Some(damage_timer) = damage_timer {
        if damage_timer.timer.finished() {
            commands.remove_resource::<FightLostTimer>();
            *player_status = PlayerStatus::default();
            next_fight_state.set(FightState::default());
            next_ingame_state.set(InGameState::default());
            next_app_state.set(AppState::MainMenu);
        }
    }
}

pub fn clear_combat_log(
    mut combat_log: ResMut<CombatLog>
) {
    combat_log.logs.clear();
}

pub fn reset_player_is_defending(
    mut player_defending: ResMut<PlayerIsDefending>
) {
    player_defending.0 = false;
}