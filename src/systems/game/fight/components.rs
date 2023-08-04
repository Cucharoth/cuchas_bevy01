use crate::prelude::*;

#[derive(Component, Debug)]
pub struct Enemy{
    pub health: f32,
    pub damage: f32,
    pub speed: f32
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
