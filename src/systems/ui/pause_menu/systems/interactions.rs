use bevy::prelude::*;
use bevy_ui_navigation::prelude::{Focusable, FocusState};

use crate::systems::ui::{resources::ButtonFocusChangeAudio, plays_focus_change_audio};

use super::style::*;



pub fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<(&Focusable, &mut BackgroundColor, &mut Text), Changed<Focusable>>,
    focus_change_audio: Res<ButtonFocusChangeAudio>
) {
    for (focusable, mut material, mut text) in interaction_query.iter_mut() {
        if let FocusState::Focused = focusable.state() {
            plays_focus_change_audio(&mut commands, &focus_change_audio);
            //*material = MAIN_MENU_PRESSED_BUTTON_COLOR.into();
            text.sections[0].style.color = PAUSE_MENU_FOCUSED_BUTTON_TEXT.into();
        } else {
            //*material = MAIN_MENU_HOVERED_BUTTON_COLOR.into();
            text.sections[0].style.color = PAUSE_MENU_BUTTON_TEXT_COLOR.into();
        }
    }
}