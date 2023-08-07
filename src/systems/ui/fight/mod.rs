pub mod events;
pub mod systems;
pub mod resources;

use bevy::prelude::*;
use bevy_ui_navigation::NavRequestSystem;

use crate::prelude::{fight::in_fight::FightState, GameState};
use crate::prelude::InGameState;
use crate::systems::game::fight::turns::events::*;
use crate::systems::ui::fight::systems::interaction::interactions::*;
use crate::systems::ui::fight::systems::layout::*;
use crate::AppState;

use self::{events::{ReFocusButtonEvent, HidePlayerSkillList}, resources::SaraSprites};
pub struct FightUIPlugin;

impl Plugin for FightUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDamageEvent>()
            .add_event::<EnemyDamageEvent>()
            .add_event::<ReFocusButtonEvent>()
            .add_event::<HidePlayerSkillList>()
            .add_plugins(CombatLogTextPlugin)
            .add_systems(Startup, add_extra_sprites)
            // intro
            .add_systems(OnEnter(FightState::Intro), (create_fight_ui, combat_log_root))
            // damage happening
            .add_systems(
                Update,
                (update_hp_enemy_node, update_mp_player_status_node, update_hp_player_status_node).run_if(in_state(InGameState::Fight)),
            )
            .add_systems(
                OnExit(FightState::Intro),
                (show_status_ui)
                    .run_if(in_state(InGameState::Fight))
                    .run_if(in_state(AppState::Game)),
            )
            // player turn
            .add_systems(
                OnEnter(FightState::PlayerTurn),
                (
                    update_hp_player_status_node,
                    update_mp_player_status_node,
                    last_turn_update,
                    show_player_ui,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(InGameState::Fight)),
            )
            .add_systems(
                Update,
                (
                    interact_with_attack_button,
                    interact_with_skill_button.after(NavRequestSystem),
                    interact_with_defend_button,
                    interact_with_escape_button,
                    interact_with_skill_list_button,
                    back_from_skill_list,
                    re_focus_button_handler.after(NavRequestSystem),
                    button_system.after(NavRequestSystem).after(interact_with_skill_button)
                )
                    .run_if(in_state(FightState::PlayerTurn))
                    .run_if(in_state(InGameState::Fight))
                    //.run_if(in_state(GameState::Running))
            )
            .add_systems(
                OnExit(FightState::PlayerTurn),
                (hide_player_ui)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(InGameState::Fight)),
            )
            // damage to player
            .add_systems(
                OnTransition {
                    from: FightState::DamageHappening,
                    to: FightState::PlayerTurn,
                },
                (show_player_ui)
                    .run_if(in_state(InGameState::Fight))
                    .run_if(in_state(AppState::Game)),
            )
            // player to damage
            .add_systems(OnTransition {
                from: FightState::PlayerTurn,
                to: FightState::DamageHappening
            }, hide_psl_event_handler)
            // damage to enemy
            .add_systems(OnTransition {
                from: FightState::DamageHappening,
                to: FightState::EnemyTurn
            }, hide_sara_cast)
            // exit fight
            .add_systems(OnExit(InGameState::Fight), (despawn_fight_state, reset_sara_sprites_1, reset_sara_sprites_2).run_if(in_state(AppState::Game)));
        //damage happening
        /* .add_systems(
            Update,
            ()
            .run_if(in_state(InGameState::Fight))
        )*/
        //enemy turn
    }
}


fn add_extra_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.insert_resource(SaraSprites {
        sprites: vec![
            asset_server.load("sara_cast.png"),
            asset_server.load("sara_cast_succesful.png"),
            asset_server.load("sara_dedge.png")
        ]
    });
}