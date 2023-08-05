pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

use crate::systems::game::fight::turns::resources::*;
use crate::systems::ui::fight::systems::interaction::interactions::update_hp_player_status_node;
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
                damage_happening_ticker,
                damage_happening_timer_check,
                player_does_damage_check,
                enemy_does_damage_check,
                check_if_enemy_is_dead,
                check_if_player_is_dead.after(update_hp_player_status_node)
            )
                .run_if(in_state(FightState::DamageHappening))
                .run_if(in_state(AppState::Game))
                .chain()
        )
        // enemy turn
        .add_systems(
            Update,
            enemy_turn
                .run_if(in_state(FightState::EnemyTurn))
                .run_if(in_state(AppState::Game)),
        )
        .add_systems(OnTransition {
            from: FightState::DamageHappening,
            to: FightState::EnemyTurn
        }, check_if_enemy_is_dead)
        // win
        .add_systems(OnEnter(FightState::Win), fight_win_timer)
        .add_systems(Update, (fight_win_ticker, win_timer_check).run_if(in_state(FightState::Win)))
        // lost
        .add_systems(OnEnter(FightState::Lost), fight_lost_timer)
        .add_systems(Update, (fight_lost_ticker, fight_lost_timer_check).run_if(in_state(FightState::Lost)));
    }
}
