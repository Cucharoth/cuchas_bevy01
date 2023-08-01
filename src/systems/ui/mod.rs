pub mod main_menu;
use crate::systems::ui::main_menu::menu::MainMenuPlugin;
use bevy::prelude::*;
use bevy_ui_navigation::{DefaultNavigationPlugins, systems::InputMapping};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_key_mapping)
        .add_plugins((DefaultNavigationPlugins, MainMenuPlugin));
    }
}

fn set_key_mapping(
    mut input_mapping: ResMut<InputMapping>,
) {
    input_mapping.key_action = KeyCode::Z;
    input_mapping.key_cancel = KeyCode::X;

    //println!("{:?}", input_mapping.key_free);
}