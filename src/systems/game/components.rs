use crate::prelude::*;

#[derive(Component)]
pub struct Player {
    pub health: u32,
    pub damage: u32,
    pub mov_speed: f32,
    pub speed: u32
}

impl Default for Player {
    fn default() -> Self {
        Self { health: 100, damage: 10, mov_speed: 500.0, speed: 100 }
    }
}