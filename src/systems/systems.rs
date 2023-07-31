
use crate::prelude::*;

pub fn transition_to_game_state(
    //mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::G){
        let current_app_state = app_state.get();
        if *current_app_state != AppState::Game {
            //commands.insert_resource(NextState(Some(AppState::Game)));
            next_app_state.set(AppState::Game);
            println!("Entered AppState::Game")
        }
        
    }
}

pub fn transition_to_menu_state(
    //mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,  
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_ingame_state: ResMut<NextState<InGameState>>
) {
    if keyboard_input.just_pressed(KeyCode::M){
        let current_app_state = app_state.get();
        if *current_app_state != AppState::MainMenu {
            //commands.insert_resource(NextState(Some(AppState::MainMenu)));
            next_app_state.set(AppState::MainMenu);
            next_game_state.set(GameState::Paused);
            next_ingame_state.set(InGameState::WorldMap);
            println!("Entered AppState::MainMenu")
        }
        
    }
}