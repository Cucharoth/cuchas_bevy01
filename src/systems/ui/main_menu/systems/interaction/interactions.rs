use crate::systems::ui::main_menu::systems::components::*;
use crate::systems::ui::main_menu::systems::style::*;
use crate::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_ui_navigation::prelude::FocusAction;
use bevy_ui_navigation::prelude::NavRequest;
use bevy_ui_navigation::{
    prelude::{DefaultNavigationPlugins, FocusState, Focusable, NavEvent, NavRequestSystem, },
    systems::InputMapping,
    
};

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
    mut interaction_query: Query<(&Focusable, &mut BackgroundColor, &mut Text), Changed<Focusable>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for (focusable, mut material, mut text) in interaction_query.iter_mut() {
        if let FocusState::Focused = focusable.state() {
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
    mut button_interaction_query: Query<Entity, With<MainMenuPlayText>>,
    mut events: EventReader<NavEvent>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    if let Ok(button_entity) = button_interaction_query.get_single() {
        for event in events.iter() {
            if event.is_activated(button_entity) {
                next_app_state.set(AppState::Game);
            }
        }
    }
}

pub fn interact_with_exit_button(
    mut button_interaction_query: Query<Entity, With<MainMenuExitText>>,
    mut events: EventReader<NavEvent>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut exit_event_writer: EventWriter<AppExit>
) {
    if let Ok(button_entity) = button_interaction_query.get_single() {
        for event in events.iter() {
            if event.is_activated(button_entity) {
                exit_event_writer.send(AppExit);
            }
        }
    }
}

