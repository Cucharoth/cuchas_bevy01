use crate::prelude::*;

#[derive(Component)]
pub struct Player {
    pub health: f32,
    pub mana: f32,
    pub damage: f32,
    pub mov_speed: f32,
    pub speed: f32
}

impl Default for Player {
    fn default() -> Self {
        Self { health: 100., mana: 100., damage: 10., mov_speed: 500.0, speed: 100. }
    }
}
