use bevy_ui_navigation::NavRequestSystem;

use crate::prelude::*;
use crate::systems::ui::main_menu::systems::interaction::interactions::*;
use crate::systems::ui::main_menu::systems::layout::*;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
            .add_systems(Update, (
                button_system,
                interact_with_play_button,
                interact_with_exit_button,
            ).after(NavRequestSystem));
    }
}



