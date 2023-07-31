use crate::prelude::*;

#[derive(Component)]
pub struct Enemy{
    pub health: u32,
    pub damage: u32,
    pub speed: u32
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
