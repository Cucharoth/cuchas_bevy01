use bevy::utils::HashMap;

use crate::prelude::*;

#[derive(Component, Debug)]
pub struct Enemy{
    pub health: f32,
    pub damage: f32,
    pub speed: f32,
    pub debuffs: HashMap<Debuff, (f32, f32)>
}

pub struct Player{
    health: u32,
    damage: u32,
    
}

#[derive(Component)]
pub struct Movement{
    pub direction: Vec2,
    pub speed: f32
}

#[derive(Component)]
pub struct BattleTheme;

#[derive(Component)]
pub struct FightBackGround;

#[derive(Component)]
pub struct FightPlayer;

#[derive(Component)]
pub struct FightEnemy;

#[derive(Component, Debug, Clone)]
pub struct PlayerSkill {
    pub name: String,
    pub damage: f32,
    pub mana_cost: f32,
    pub effect: Option<Debuff>,
    pub effect_duration: Option<f32>
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Debuff{
    Burning,
    Freezing,
    Blindness
}

#[derive(Component)]
pub struct SaraCast;

#[derive(Component)]
pub struct SaraCastSuccesful;

#[derive(Component)]
pub struct SaraDedge;

#[derive(Component)]
pub struct ExtraSprite;