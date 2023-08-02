pub mod fight;
pub mod systems;
pub mod world_map;
pub mod components;
pub mod player;
pub mod resources;
use self::{systems::toggle_game_state, fight::in_fight::FightPlugin};
use resources::*;
use crate::prelude::*;
use world_map::in_world_map::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_state::<InGameState>()
            .init_resource::<PlayerStatus>()
            .add_plugins((
                WorldMapPlugin,
                FightPlugin,
            ))
            .add_systems(Update, toggle_game_state.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    Running,
    #[default]
    Paused,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum InGameState {
    #[default]
    WorldMap,
    Fight,
}
