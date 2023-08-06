pub mod style;
pub mod layout;
pub mod interactions;
pub mod components;

use bevy::prelude::*;
use crate::systems::ui::pause_menu::systems::layout::*;
use crate::systems::ui::pause_menu::systems::interactions::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, pause_menu_root)
        .add_systems(Update, button_system);
    }
}