pub mod events;
pub mod systems;

use bevy::prelude::*;

use crate::prelude::fight::in_fight::FightState;
use crate::prelude::InGameState;
use crate::systems::game::fight::turns::events::*;
use crate::systems::ui::fight::systems::interaction::interactions::*;
use crate::systems::ui::fight::systems::layout::*;
pub struct FightUIPlugin;

impl Plugin for FightUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDamageEvent>()
            .add_event::<EnemyDamageEvent>()
            .add_systems(OnEnter(FightState::Intro), (create_fight_ui))
            .add_systems(Update, (button_system, update_hp_enemy_node).run_if(in_state(InGameState::Fight)))
            .add_systems(
                OnEnter(FightState::PlayerTurn),
                (update_hp_status_node, update_mp_status_node, last_turn_update),
            )
            // player turn
            .add_systems(
                Update,
                interact_with_attack_button
                    .run_if(in_state(FightState::PlayerTurn))
                    .run_if(in_state(InGameState::Fight)),
            );
            //damage happening
            /* .add_systems(
                Update,
                ()
                .run_if(in_state(InGameState::Fight))
            )*/
        //enemy turn
    }
}
