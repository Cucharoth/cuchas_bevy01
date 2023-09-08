//#![windows_subsystem = "windows"]
mod events;
mod resources;
mod systems;

mod prelude {
    pub use crate::events::*;
    pub use crate::resources::*;
    pub use crate::systems::game::world_map::in_world_map::*;
    pub use crate::systems::game::*;
    pub use crate::systems::ui::main_menu::menu::*;
    pub use crate::*;
    pub use bevy::prelude::*;
}

use bevy::winit::WinitWindows;
use bevy::window::PrimaryWindow;
use winit::window::Icon;
use prelude::*;
use systems::{systems::DevPlugin, ui::UiPlugin};

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some( Window {
                        title: "Cucha's RPG".into(),
                        resizable: false,
                        resize_constraints: WindowResizeConstraints {
                            max_width: 1280.,
                            max_height: 720.,
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .build(),
            GamePlugin,
            UiPlugin,
            //DevPlugin,
        ))
        .add_systems(Startup, (spawn_camera, set_window_icon))
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

pub fn set_window_icon(
    main_window: Query<Entity, With<PrimaryWindow>>,
    windows: NonSend<WinitWindows>,
) {
    let Some(primary) = windows.get_window(main_window.single()) else {return};

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("icon.ico")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    primary.set_window_icon(Some(icon));
}