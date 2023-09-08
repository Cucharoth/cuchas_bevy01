pub mod components;
pub mod interactions;
pub mod layout;
pub mod style;
pub mod events;

use crate::prelude::GameState;
use crate::systems::ui::fight::systems::interaction::interactions::button_system as fight_button_system;
use crate::systems::ui::main_menu::systems::interaction::interactions::button_system as main_menu_button_system;
use crate::systems::ui::pause_menu::systems::interactions::*;
use crate::systems::ui::pause_menu::systems::layout::*;
use crate::AppState;
use bevy::prelude::*;

use self::events::ReFocusPauseMenuEvent;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ReFocusPauseMenuEvent>()
        //.add_systems(Startup, create_pause_menu_root)
            .add_systems(Update, (update_focus_color))
            .add_systems(
                Update,
                (
                    button_system,
                    interact_with_resume_button,
                    interact_with_main_menu_button,
                )
                    .run_if(in_state(GameState::Paused))
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Paused)),
            )
            .add_systems(OnTransition {
                from: GameState::Running,
                to: GameState::Paused
            }, create_pause_menu_root)
            // on enter pause
            .add_systems(
                OnEnter(GameState::Paused),
                (
                    button_system,
                )
                    .run_if(in_state(AppState::Game))
                    .chain()
            )
            .add_systems(
                OnEnter(GameState::Paused),
                (
                    show_pause_menu
                )
                    .run_if(in_state(AppState::Game))
                    .after(create_pause_menu_root),
            )
            // on exit pause
            .add_systems(
                OnExit(GameState::Paused),
                (hide_pause_menu, despawn_pause_menu).run_if(in_state(AppState::Game)).chain(),
            )
            // on exit game
            .add_systems(OnExit(AppState::Game), despawn_pause_menu);
    }
}
