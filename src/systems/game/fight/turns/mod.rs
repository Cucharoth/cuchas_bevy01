pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

use crate::systems::game::fight::turns::resources::*;
use crate::{systems::game::fight::turns::systems::*, AppState};
use bevy::prelude::*;

use super::in_fight::FightState;
pub struct TurnsPlugin;

impl Plugin for TurnsPlugin {
    fn build(&self, app: &mut App) {
        // damage happening
        app.init_resource::<PlayerIsDefending>()
        .add_systems(
            OnEnter(FightState::DamageHappening),
            (damage_happening_timer).run_if(in_state(AppState::Game)),
        )
        .add_systems(
            Update,
            (
                player_does_damage_check,
                damage_happening_ticker,
                damage_happening_timer_check,
                enemy_does_damage_check,
            )
                .run_if(in_state(FightState::DamageHappening))
                .run_if(in_state(AppState::Game)),
        )
        .add_systems(
            Update,
            enemy_turn
                .run_if(in_state(FightState::EnemyTurn))
                .run_if(in_state(AppState::Game)),
        );
    }
}
