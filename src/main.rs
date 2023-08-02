mod events;
mod resources;
mod systems;

mod prelude {
    pub use crate::events::*;
    pub use crate::resources::*;
    pub use crate::systems::game::world_map::in_world_map::*;
    pub use crate::systems::ui::main_menu::menu::*;
    pub use crate::systems::game::*;
    pub use bevy::prelude::*;
    pub use crate::*;
}

use prelude::*;
use systems::{systems::{DevPlugin}, ui::UiPlugin};


fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()).build(),
            GamePlugin,
            UiPlugin,
            DevPlugin
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver
}