pub mod components;
pub mod interactions;
pub mod layout;
pub mod style;

use crate::prelude::GameState;
use crate::systems::ui::pause_menu::systems::interactions::*;
use crate::systems::ui::pause_menu::systems::layout::*;
use crate::AppState;
use bevy::prelude::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, pause_menu_root)
            .add_systems(
                Update,
                (
                    //button_system,
                    interact_with_resume_button,
                    interact_with_main_menu_button,
                )
                    .run_if(in_state(GameState::Paused))
                    .run_if(in_state(AppState::Game)),
            );
            // on enter pause
            /*.add_systems(
                OnEnter(GameState::Paused),
                (show_pause_menu, button_system).run_if(in_state(AppState::Game)),
            )*/
            // on exit pause
            /*.add_systems(
                OnExit(GameState::Paused),
                hide_pause_menu.run_if(in_state(AppState::Game)),
            )*/
            // on exit game
            //.add_systems(OnExit(AppState::Game), hide_pause_menu);
    }
}
