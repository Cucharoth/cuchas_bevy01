use bevy::audio::*;
use bevy_ui_navigation::NavRequestSystem;

use crate::prelude::*;
use crate::systems::ui::main_menu::components::*;
use crate::systems::ui::main_menu::systems::interaction::interactions::*;
use crate::systems::ui::main_menu::systems::layout::*;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::MainMenu),
            (spawn_main_menu, main_menu_theme),
        )
        .add_systems(
            OnExit(AppState::MainMenu),
            (despawn_main_menu, despawn_main_menu_theme),
        )
        .add_systems(
            Update,
            (
                button_system,
                interact_with_play_button,
                interact_with_exit_button,
            )
                .after(NavRequestSystem),
        );
    }
}

fn main_menu_theme(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("Audio/2-10 The Royal Palace.wav"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::Relative(VolumeLevel::new(0.5)),
                ..Default::default()
            },
        },
        MainMenuTheme,
    ));
}

fn despawn_main_menu_theme(
    mut commands: Commands,
    main_menu_theme_q: Query<(Entity, &AudioSink), With<MainMenuTheme>>,
) {
    let (main_menu_theme_entity, main_menu_theme) = main_menu_theme_q.get_single().unwrap();
    main_menu_theme.stop();
    commands.entity(main_menu_theme_entity).despawn();
}
