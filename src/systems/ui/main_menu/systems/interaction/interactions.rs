use crate::prelude::InGameState;
use crate::systems::ui::main_menu::systems::components::*;
use crate::systems::ui::main_menu::systems::style::*;
use crate::AppState;
use crate::systems::ui::plays_button_in_audio;
use crate::systems::ui::plays_button_out_audio;
use crate::systems::ui::plays_focus_change_audio;
use crate::systems::ui::resources::*;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_ui_navigation::prelude::{FocusState, Focusable, NavEvent};

pub fn old_interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = MAIN_MENU_PRESSED_BUTTON_COLOR.into();
                next_app_state.set(AppState::Game);
            }
            Interaction::Hovered => {
                *background_color = MAIN_MENU_HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = MAIN_MENU_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn old_interact_with_exit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ExitButton>),
    >,
    mut exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = MAIN_MENU_PRESSED_BUTTON_COLOR.into();
                exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => *background_color = MAIN_MENU_HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = MAIN_MENU_BUTTON_COLOR.into(),
        }
    }
}

pub fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<(&Focusable, &mut BackgroundColor, &mut Text), Changed<Focusable>>,
    focus_change_audio: Res<ButtonFocusChangeAudio>
) {
    for (focusable, mut material, mut text) in interaction_query.iter_mut() {
        if let FocusState::Focused = focusable.state() {
            plays_focus_change_audio(&mut commands, &focus_change_audio);
            //*material = MAIN_MENU_PRESSED_BUTTON_COLOR.into();
            text.sections[0].style.color = MAIN_MENU_SELECTED_TEXT.into();
        } else {
            //*material = MAIN_MENU_HOVERED_BUTTON_COLOR.into();
            text.sections[0].style.color = MAIN_MENU_IDDLE_TEXT.into();
        }
    }
}

pub fn print_nav_events(mut events: EventReader<NavEvent>) {
    for event in events.iter() {
        println!("{:?}", event);
    }
}

pub fn interact_with_play_button(
    mut commands: Commands,
    button_interaction_query: Query<Entity, With<MainMenuPlayText>>,
    mut events: EventReader<NavEvent>,
    mut next_ingame_state: ResMut<NextState<InGameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    button_in_audio: Res<ButtonInAudio>
) {
    if let Ok(button_entity) = button_interaction_query.get_single() {
        for event in events.iter() {
            if event.is_activated(button_entity) {
                plays_button_in_audio(&mut commands, &button_in_audio);
                next_app_state.set(AppState::Game);
                next_ingame_state.set(InGameState::WorldMap);
            }
        }
    }
}

pub fn interact_with_exit_button(
    mut commands: Commands,
    button_interaction_query: Query<Entity, With<MainMenuExitText>>,
    mut events: EventReader<NavEvent>,
    mut exit_event_writer: EventWriter<AppExit>,
    button_out_audio: Res<ButtonOutAudio>
) {
    if let Ok(button_entity) = button_interaction_query.get_single() {
        for event in events.iter() {
            if event.is_activated(button_entity) {
                plays_button_out_audio(&mut commands, &button_out_audio);
                exit_event_writer.send(AppExit);
            }
        }
    }
}

