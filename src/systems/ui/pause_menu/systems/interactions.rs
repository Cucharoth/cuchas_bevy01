use bevy::prelude::*;
use bevy_ui_navigation::prelude::{FocusState, Focusable, NavEvent, NavRequest};

use super::style::*;
use crate::prelude::fight::in_fight::FightState;
use crate::prelude::{GameState, InGameState};
use crate::systems::ui::fight::events::ReFocusButtonEvent;
use crate::systems::ui::fight::systems::components::*;
use crate::systems::ui::fight::systems::interaction::interactions::{
    blocks_main_ui, unlock_player_buttons,
};
use crate::systems::ui::pause_menu::systems::components::*;
use crate::systems::ui::{plays_focus_change_audio, resources::ButtonFocusChangeAudio};
use crate::AppState;

pub fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<(&Focusable, &mut Text), Changed<Focusable>>,
    focus_change_audio: Res<ButtonFocusChangeAudio>,
) {
    for (focusable, mut text) in interaction_query.iter_mut() {
        if let FocusState::Focused = focusable.state() {
            plays_focus_change_audio(&mut commands, &focus_change_audio);
            text.sections[0].style.color = PAUSE_MENU_FOCUSED_BUTTON_TEXT.into();
        } else {
            text.sections[0].style.color = PAUSE_MENU_BUTTON_TEXT_COLOR.into();
        }
    }
}

pub fn interact_with_resume_button(
    resumen_button_q: Query<Entity, With<PauseMenuResumeText>>,
    mut nav_event_reader: EventReader<NavEvent>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(resumen_button_entity) = resumen_button_q.get_single() {
        for event in nav_event_reader.iter() {
            if event.is_activated(resumen_button_entity) {
                next_game_state.set(GameState::Running);
            }
        }
    }
}

pub fn interact_with_main_menu_button(
    resumen_button_q: Query<Entity, With<PauseMenuMainMenuText>>,
    mut nav_event_reader: EventReader<NavEvent>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_ingame_state: ResMut<NextState<InGameState>>,
    mut next_fight_state: ResMut<NextState<FightState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(resumen_button_entity) = resumen_button_q.get_single() {
        for event in nav_event_reader.iter() {
            if event.is_activated(resumen_button_entity) {
                next_app_state.set(AppState::MainMenu);
                next_ingame_state.set(InGameState::default());
                next_fight_state.set(FightState::default());
                next_game_state.set(GameState::default());
            }
        }
    }
}

pub fn show_pause_menu(
    mut pause_menu_text_q: Query<&mut Focusable, With<PauseText>>,
    mut resumen_button_q: Query<(Entity, &mut Text), With<PauseMenuResumeText>>,
    mut pause_menu_node_q: Query<
        &mut Visibility,
        (With<PauseMenuNode>, Without<PlayerButtonsNode>),
    >,
    mut player_buttons_q: Query<&mut Children, (With<PlayerButtonsNode>, Without<SkillListNode>)>,
    mut focusable_q: Query<&mut Focusable, Without<PauseText>>,
    mut nav_event: EventWriter<NavRequest>,
) {
    if let Ok(mut pause_menu_vis) = pause_menu_node_q.get_single_mut() {
        *pause_menu_vis = Visibility::Visible;
        blocks_main_ui(&mut player_buttons_q, &mut focusable_q);

        for mut text_focusable in pause_menu_text_q.iter_mut() {
            println!("desblockeando");
            text_focusable.unblock();
        }

        if let Ok((resume_button_entity, mut text)) = resumen_button_q.get_single_mut() {
            text.sections[0].style.color = PAUSE_MENU_FOCUSED_BUTTON_TEXT.into();
            nav_event.send(NavRequest::FocusOn(resume_button_entity));
        }
    }
}

pub fn hide_pause_menu(
    mut pause_menu_text_q: Query<&mut Focusable, With<PauseText>>,
    mut pause_menu_node_q: Query<
        &mut Visibility,
        (With<PauseMenuNode>, Without<PlayerButtonsNode>),
    >,
    mut player_buttons_q: Query<&mut Children, (With<PlayerButtonsNode>, Without<SkillListNode>)>,
    mut focusable_q: Query<&mut Focusable, Without<PauseText>>,
    mut event_writer: EventWriter<ReFocusButtonEvent>,
) {
    if let Ok(mut pause_menu_vis) = pause_menu_node_q.get_single_mut() {
        for mut text_focusable in pause_menu_text_q.iter_mut() {
            // iterando sobre botones y no el texto!!! FIIIX
            println!("blockeando");
            *text_focusable = text_focusable.clone().blocked();
        }

        *pause_menu_vis = Visibility::Hidden;
        unlock_player_buttons(&mut player_buttons_q, &mut focusable_q, &mut event_writer);
    }
}
