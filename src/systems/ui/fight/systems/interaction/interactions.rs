use bevy::prelude::*;
use bevy_ui_navigation::prelude::{FocusState, Focusable, NavEvent};
use crate::prelude::fight::components::Enemy;
use crate::prelude::fight::in_fight::FightState;
use crate::systems::ui::fight::systems::style::*;
use crate::systems::game::resources::PlayerStatus;
use crate::systems::ui::fight::systems::components::*;
use crate::systems::game::fight::turns::events::*;
use crate::systems::game::fight::turns::systems::*;
use crate::systems::game::fight::turns::resources::*;


pub fn button_system(
    mut interaction_query: Query<
        (&Focusable, &mut BackgroundColor, &mut BorderColor),
        Changed<Focusable>,
    >,
    keyboard_input: Res<Input<KeyCode>>,
    //mut next_app_state: ResMut<NextState<AppState>>,
) {
    for (focusable, mut background_color, mut border_color) in interaction_query.iter_mut() {
        if let FocusState::Focused = focusable.state() {
            *border_color = FIGHT_UI_FOCUSED_NODE_COLOR.into();
            //*background_color = Color::RED.into();
            //text.sections[0].style.color = MAIN_MENU_SELECTED_TEXT.into();
        } else {
            *border_color = FIGHT_UI_IDDLE_NODE_COLOR.into();
            //*background_color = Color::BEIGE.into();
            //text.sections[0].style.color = MAIN_MENU_IDDLE_TEXT.into();
        }
    }
}

pub fn update_hp_enemy_node(
    mut enemy_q: Query<&Enemy, With<Enemy>>,
    mut enemy_node_q: Query<&mut Text, With<FightEnemyHP>>
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
    player_status: Res<PlayerStatus>
) {
    if let Ok(mut node_hp_status) = node_status_hp_query.get_single_mut() {
        let text_input = format!("HP: \t {}", player_status.health);
        node_hp_status.sections[0].value = text_input;
    }
}

pub fn update_mp_status_node(
    mut node_status_mp_query: Query<&mut Text, With<FightStatusMP>>,
    player_status: Res<PlayerStatus>
) {
    if let Ok(mut node_mp_status) = node_status_mp_query.get_single_mut() {
        let text_input = format!("MP: \t {}", player_status.mana);
        node_mp_status.sections[0].value = text_input;
    }
}

pub fn interact_with_attack_button(
    attack_button_q: Query<Entity, With<FightAttackButton>>,
    mut events: EventReader<NavEvent>,
    player_status: Res<PlayerStatus>,
    mut event_writter: EventWriter<PlayerDamageEvent>,
    mut next_fight_state: ResMut<NextState<FightState>>
) {
    if let Ok(button_entity) = attack_button_q.get_single() {
        for event in events.iter() {
            if event.is_activated(button_entity) {
                player_attack(&player_status, &mut event_writter);
                next_fight_state.set(FightState::DamageHappening);
                println!("DAMAGE HAPPENING");
            }
        }
    }
}

pub fn last_turn_update(
    mut player_active_last_turn: ResMut<PlayerActiveLastTurn>
) {
    player_active_last_turn.0 = true;
}
