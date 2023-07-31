
use crate::prelude::*;

pub fn toggle_game_state(
    mut commands: Commands,
    keyboar_input: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>
){
    if keyboar_input.just_pressed(KeyCode::Escape) {
        let current_game_state = game_state.get();
        if *current_game_state == GameState::Running{
            commands.insert_resource(NextState(Some(GameState::Paused)));
            println!("GAME PAUSED");
        }
        if *current_game_state == GameState::Paused{
            commands.insert_resource(NextState(Some(GameState::Running)));
            println!("GAME RUNNING");
        }
    }
}