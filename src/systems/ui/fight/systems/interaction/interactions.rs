use crate::prelude::fight::components::Enemy;
use crate::prelude::fight::components::PlayerSkill;
use crate::prelude::fight::in_fight::FightState;
use crate::prelude::world_map::enemy::spawn_single_enemy;
use crate::prelude::InGameState;
use crate::systems::game::fight::turns::events::*;
use crate::systems::game::fight::turns::resources::*;
use crate::systems::game::fight::turns::systems::*;
use crate::systems::game::resources::PlayerStatus;
use crate::systems::ui::fight::events::*;
use crate::systems::ui::fight::systems::components::*;
use crate::systems::ui::fight::systems::style::*;
use crate::systems::ui::resources::*;
use crate::systems::ui::{plays_button_in_audio, plays_button_out_audio, plays_focus_change_audio};
use bevy::audio;
use bevy::audio::PlaybackMode;
use bevy::ecs::event;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ui_navigation::prelude::NavRequest;
use bevy_ui_navigation::prelude::{FocusState, Focusable, NavEvent};
use rand::prelude::*;

pub fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Focusable, &mut BackgroundColor, &mut BorderColor, Entity),
        Changed<Focusable>,
    >,
    keyboard_input: Res<Input<KeyCode>>,
    mut events: EventReader<NavEvent>,
    //focus_change_audio_q: Query<&AudioSink, With<ButtonFocusChangeAudio>>,
    focus_change_audio: Res<ButtonFocusChangeAudio>,
) {
    for (focusable, mut background_color, mut border_color, entity) in interaction_query.iter_mut()
    {
        if let FocusState::Focused = focusable.state() {
            plays_focus_change_audio(&mut commands, &focus_change_audio);

            *border_color = FIGHT_UI_FOCUSED_NODE_COLOR.into();
            *background_color = FIGHT_UI_BUTTON_COLOR.into();

            //text.sections[0].style.color = MAIN_MENU_SELECTED_TEXT.into();
        } else {
            *border_color = FIGHT_UI_IDDLE_NODE_COLOR.into();
            *background_color = FIGHT_UI_BUTTON_COLOR.into();
            //text.sections[0].style.color = MAIN_MENU_IDDLE_TEXT.into();
        }
    }
}

pub fn update_hp_enemy_node(
    mut enemy_q: Query<&Enemy, With<Enemy>>,
    mut enemy_node_q: Query<&mut Text, With<FightEnemyHP>>,
) {
    if let Ok(mut enemy_node_text) = enemy_node_q.get_single_mut() {
        for enemy in enemy_q.iter_mut() {
            let text_input = format!("HP: \t {}", enemy.health);
            enemy_node_text.sections[0].value = text_input;
        }
    }
}

pub fn update_hp_status_node(
    mut node_status_hp_query: Query<&mut Text, With<FightStatusHP>>,
    player_status: Res<PlayerStatus>,
) {
    if let Ok(mut node_hp_status) = node_status_hp_query.get_single_mut() {
        let text_input = format!("HP: \t {}", player_status.health);
        node_hp_status.sections[0].value = text_input;
    }
}

pub fn update_mp_status_node(
    mut node_status_mp_query: Query<&mut Text, With<FightStatusMP>>,
    player_status: Res<PlayerStatus>,
) {
    if let Ok(mut node_mp_status) = node_status_mp_query.get_single_mut() {
        let text_input = format!("MP: \t {}", player_status.mana);
        node_mp_status.sections[0].value = text_input;
    }
}

pub fn interact_with_attack_button(
    mut commands: Commands,
    mut attack_button_q: Query<(Entity, &mut BackgroundColor), With<FightAttackButton>>,
    mut events: EventReader<NavEvent>,
    player_status: Res<PlayerStatus>,
    mut event_writter: EventWriter<PlayerDamageEvent>,
    mut next_fight_state: ResMut<NextState<FightState>>,
    button_in_audio: Res<ButtonInAudio>,
) {
    if let Ok((button_entity, mut background_color)) = attack_button_q.get_single_mut() {
        for event in events.iter() {
            if event.is_activated(button_entity) {
                plays_button_in_audio(&mut commands, &button_in_audio);
                *background_color = FIGHT_UI_ACTIONED_BUTTON_COLOR.into();
                player_attack(&player_status, &mut event_writter);
                next_fight_state.set(FightState::DamageHappening);
                println!("DAMAGE HAPPENING");
            }
        }
    }
}

pub fn interact_with_skill_button(
    mut commands: Commands,
    mut skill_button_q: Query<(Entity, &mut BackgroundColor), With<FightSkillButton>>,
    mut events: EventReader<NavEvent>,
    mut player_skill_list_button_q: Query<(&mut Visibility, &mut Children), With<SkillListNode>>,
    mut focusable_q: Query<&mut Focusable>,
    mut player_buttons_q: Query<&mut Children, (With<PlayerButtonsNode>, Without<SkillListNode>)>,
    mut nav_event_request: EventWriter<NavRequest>,
    button_in_audio: Res<ButtonInAudio>,
) {
    if let Ok((button_entity, mut background_color)) = skill_button_q.get_single_mut() {
        for event in events.iter() {
            if event.is_activated(button_entity) {
                plays_button_in_audio(&mut commands, &button_in_audio);
                *background_color = FIGHT_UI_ACTIONED_BUTTON_COLOR.into();
                if let Ok((mut visibility_skill_list_button, childrens)) =
                    player_skill_list_button_q.get_single_mut()
                {
                    // unblocks skill list
                    for children in childrens.iter() {
                        if let Ok(mut child_focusable) = focusable_q.get_mut(*children) {
                            child_focusable.unblock();
                        }
                    }
                    // focus on the first one in skill list
                    nav_event_request.send(NavRequest::FocusOn(childrens[0]));

                    blocks_main_ui(&mut player_buttons_q, &mut focusable_q);
                    *visibility_skill_list_button = Visibility::Visible;
                }
            }
        }
    }
}

pub fn interact_with_defend_button(
    mut commands: Commands,
    mut events: EventReader<NavEvent>,
    mut defend_button_q: Query<(Entity, &mut BackgroundColor), With<FightDefButton>>,
    button_in_audio: Res<ButtonInAudio>,
    mut next_fight_state: ResMut<NextState<FightState>>,
    mut player_defending: ResMut<PlayerIsDefending>,
) {
    if let Ok((defend_button_entity, mut background_color)) = defend_button_q.get_single_mut() {
        for event in events.iter() {
            if event.is_activated(defend_button_entity) {
                plays_button_in_audio(&mut commands, &button_in_audio);
                player_defending.0 = true;
                *background_color = FIGHT_UI_ACTIONED_BUTTON_COLOR.into();
                println!("Player is defending");
                next_fight_state.set(FightState::DamageHappening);
            }
        }
    }
}

pub fn interact_with_escape_button(
    mut commands: Commands,
    mut events: EventReader<NavEvent>,
    mut escape_button_q: Query<(Entity, &mut BackgroundColor), With<FightEscapeButton>>,
    button_out_audio: Res<ButtonOutAudio>,
    mut next_fight_state: ResMut<NextState<FightState>>,
    mut next_ingame_state: ResMut<NextState<InGameState>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut player_status: ResMut<PlayerStatus>,
) {
    if let Ok((escape_button_entity, mut background_color)) = escape_button_q.get_single_mut() {
        for event in events.iter() {
            if event.is_activated(escape_button_entity) {
                plays_button_out_audio(&mut commands, &button_out_audio);
                *background_color = FIGHT_UI_ACTIONED_BUTTON_COLOR.into();
                let mut gen_value = 0.3;
                if (gen_value + player_status.bad_luck_protection) > 1. {
                    gen_value = 1.;
                } else {
                    gen_value += player_status.bad_luck_protection;
                }
                let player_escapes = thread_rng().gen_bool(gen_value);
                if player_escapes {
                    player_status.bad_luck_protection = 0.;
                    println!("Player Escapes!");
                    next_ingame_state.set(InGameState::WorldMap);
                    spawn_single_enemy(&mut commands, &window_query, &asset_server, &player_status);
                } else {
                    player_status.bad_luck_protection += 0.2;
                    println!("Player tried to escape but the enemy was too fast!");
                    next_fight_state.set(FightState::DamageHappening);
                }
            }
        }
    }
}

pub fn interact_with_skill_list_button(
    player_skill_list_button_q: Query<&mut Children, With<SkillListNode>>,
    mut nav_event_reader: EventReader<NavEvent>,
    with_player_skill: Query<&mut PlayerSkill>,
    mut player_buttons_q: Query<&mut Children, (With<PlayerButtonsNode>, Without<SkillListNode>)>,
    mut focusable_q: Query<&mut Focusable>,
    mut dmg_event_writter: EventWriter<PlayerDamageEvent>,
    mut next_fight_state: ResMut<NextState<FightState>>,
    mut re_focus_event_writter: EventWriter<ReFocusButtonEvent>,
    mut hide_psl_event: EventWriter<HidePlayerSkillList>
) {
    if let Ok(skill_buttons) = player_skill_list_button_q.get_single() {
        for event in nav_event_reader.iter() {
            for skill_button_entity in skill_buttons.iter() {
                if event.is_activated(*skill_button_entity) {
                    let player_skill = with_player_skill.get(*skill_button_entity).unwrap();
                    println!("Player uses {}!", player_skill.name);

                    dmg_event_writter.send(PlayerDamageEvent {
                        damage: player_skill.damage,
                        debuff: player_skill.effect.clone(),
                        debuff_duration: player_skill.effect_duration,
                    });
                    hide_psl_event.send(HidePlayerSkillList{entity: *skill_button_entity});

                    unlock_player_buttons(
                        &mut player_buttons_q,
                        &mut focusable_q,
                        &mut re_focus_event_writter,
                    );

                    next_fight_state.set(FightState::DamageHappening);
                }
            }
        }
    }
}

pub fn hide_psl_event_handler(
    mut player_skill_list_button_q: Query<&mut Visibility, With<SkillListNode>>,
    focusable_q: Query<&mut Focusable>,
    mut hide_psl_event_reader: EventReader<HidePlayerSkillList>
) {
    for event in hide_psl_event_reader.iter() {
        if let Ok(button_focusable) = focusable_q.get(event.entity) {
            button_focusable.clone().blocked();
        }
    }
    if let Ok(mut player_skill_list_vis) = player_skill_list_button_q.get_single_mut() {
        *player_skill_list_vis = Visibility::Hidden;
    }
}


pub fn despawn_fight_state(
    mut commands: Commands,
    fight_node_root_q: Query<Entity, With<FightNodeRoot>>,
) {
    let fight_node_root_entity = fight_node_root_q.get_single().unwrap();
    commands.entity(fight_node_root_entity).despawn_recursive();
}

fn blocks_main_ui(
    player_buttons_q: &mut Query<&mut Children, (With<PlayerButtonsNode>, Without<SkillListNode>)>,
    focusable_q: &mut Query<&mut Focusable>,
) {
    if let Ok(player_buttons_children) = player_buttons_q.get_single() {
        for child in player_buttons_children {
            if let Ok(ref mut child_focusable) = focusable_q.get_mut(*child) {
                **child_focusable = child_focusable.clone().blocked();
            }
        }
    }
}

pub fn last_turn_update(mut player_active_last_turn: ResMut<PlayerActiveLastTurn>) {
    player_active_last_turn.0 = true;
}

pub fn hide_player_ui(
    mut player_buttons_q: Query<(&mut Visibility, &mut Children), With<PlayerButtonsNode>>,
    mut button_background_q: Query<&mut BackgroundColor>,
) {
    if let Ok((mut player_buttons_visibility, mut children)) = player_buttons_q.get_single_mut() {
        *player_buttons_visibility = Visibility::Hidden;
        for child in children.iter() {
            if let Ok(mut background_color) = button_background_q.get_mut(*child) {
                *background_color = FIGHT_UI_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn show_player_ui(
    mut player_buttons_q: Query<&mut Visibility, With<PlayerButtonsNode>>,
    mut player_skill_button_q: Query<(&mut Children), With<SkillListNode>>,
    mut focusable_q: Query<&mut Focusable>,
    attack_button_q: Query<Entity, With<FightAttackButton>>,
    mut nav_event_request: EventWriter<NavRequest>,
) {
    if let Ok(skill_button_childrens) = player_skill_button_q.get_single() {
        for child in skill_button_childrens.iter() {
            if let Ok(mut child_focusable) = focusable_q.get_mut(*child) {
                child_focusable.block();
            }
        }
    }
    if let Ok(attack_button_entity) = attack_button_q.get_single() {
        nav_event_request.send(NavRequest::FocusOn(attack_button_entity));
    }
    if let Ok(mut player_buttons_visibility) = player_buttons_q.get_single_mut() {
        *player_buttons_visibility = Visibility::Visible;
    }
}

pub fn back_from_skill_list(
    mut commands: Commands,
    mut nav_request_event: EventReader<NavRequest>,
    mut player_skill_list_button_q: Query<(&mut Visibility, &mut Children), With<SkillListNode>>,
    mut player_buttons_q: Query<&mut Children, (With<PlayerButtonsNode>, Without<SkillListNode>)>,
    mut focusable_q: Query<&mut Focusable>,
    mut event_writter: EventWriter<ReFocusButtonEvent>,
    button_out_audio: Res<ButtonOutAudio>,
) {
    if let Ok((skill_list_visibility, _childrens)) = player_skill_list_button_q.get_single() {
        if skill_list_visibility == Visibility::Visible {
            for event in nav_request_event.iter() {
                if *event == NavRequest::Cancel {
                    plays_button_out_audio(&mut commands, &button_out_audio);
                    lock_skill_list(&mut player_skill_list_button_q, &mut focusable_q);
                    unlock_player_buttons(
                        &mut player_buttons_q,
                        &mut focusable_q,
                        &mut event_writter,
                    );
                }
            }
        }
    }
}

fn unlock_player_buttons(
    player_buttons_q: &mut Query<&mut Children, (With<PlayerButtonsNode>, Without<SkillListNode>)>,
    focusable_q: &mut Query<&mut Focusable>,
    event_writer: &mut EventWriter<ReFocusButtonEvent>,
) {
    if let Ok(childrens) = player_buttons_q.get_single_mut() {
        for child in childrens.iter() {
            if let Ok(mut child_focusable) = focusable_q.get_mut(*child) {
                child_focusable.unblock();
            }
        }
        event_writer.send(ReFocusButtonEvent(childrens[1]));
    }
}

fn lock_skill_list(
    player_skill_list_button_q: &mut Query<(&mut Visibility, &mut Children), With<SkillListNode>>,
    focusable_q: &mut Query<&mut Focusable>,
) {
    if let Ok((mut skill_list_visibility, childrens)) = player_skill_list_button_q.get_single_mut()
    {
        for child in childrens.iter() {
            if let Ok(mut child_focusable) = focusable_q.get_mut(*child) {
                *child_focusable = child_focusable.clone().blocked();
            }
            *skill_list_visibility = Visibility::Hidden;
        }
    }
}

pub fn re_focus_button_handler(
    mut event_reader: EventReader<ReFocusButtonEvent>,
    mut nav_writer_event: EventWriter<NavRequest>,
) {
    for event in event_reader.iter() {
        nav_writer_event.send(NavRequest::FocusOn(event.0))
    }
}

pub fn show_status_ui(mut status_ui_q: Query<&mut Visibility, With<StatusUI>>) {
    for mut ui_node_visibility in status_ui_q.iter_mut() {
        *ui_node_visibility = Visibility::Visible;
    }
}
