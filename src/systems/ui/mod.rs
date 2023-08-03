pub mod fight;
pub mod main_menu;
pub mod resources;
pub mod components;
use crate::systems::ui::main_menu::menu::MainMenuPlugin;
use crate::systems::ui::fight::FightUIPlugin;
//use crate::systems::ui::components::*;
use crate::systems::ui::resources::*;
use bevy::prelude::*;
use bevy::audio::*;
use bevy_ui_navigation::{DefaultNavigationPlugins, systems::InputMapping};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (set_key_mapping, set_key_sounds))
        .add_plugins((DefaultNavigationPlugins, MainMenuPlugin, FightUIPlugin));
    }
}

fn set_key_mapping(
    mut input_mapping: ResMut<InputMapping>,
) {
    input_mapping.key_action = KeyCode::Z;
    input_mapping.key_cancel = KeyCode::X;

    //println!("{:?}", input_mapping.key_free);
}

fn set_key_sounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.insert_resource(ButtonFocusChangeAudio {
        audio: asset_server.load("Audio/Effects/button_change.wav")
    });
    commands.insert_resource(ButtonInAudio {
        audio: asset_server.load("Audio/Effects/button_in.wav")
    });
    commands.insert_resource(ButtonOutAudio {
        audio: asset_server.load("Audio/Effects/button_out.wav")
    });
}

pub fn plays_focus_change_audio(
    commands: &mut Commands,
    focus_change_audio: &Res<ButtonFocusChangeAudio>
){
    commands.spawn( 
        AudioBundle {
            source: focus_change_audio.audio.clone(),
            settings: PlaybackSettings { mode: PlaybackMode::Despawn, ..Default::default() }
        }
    );
}

pub fn plays_button_in_audio(
    commands: &mut Commands,
    button_in_audio: &Res<ButtonInAudio>
){
    commands.spawn( 
        AudioBundle {
            source: button_in_audio.audio.clone(),
            settings: PlaybackSettings { mode: PlaybackMode::Despawn, ..Default::default() }
        }
    );
}

pub fn plays_button_out_audio(
    commands: &mut Commands,
    button_out_audio: &Res<ButtonOutAudio>
){
    commands.spawn( 
        AudioBundle {
            source: button_out_audio.audio.clone(),
            settings: PlaybackSettings { mode: PlaybackMode::Despawn, ..Default::default() }
        }
    );
}