use crate::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub damage: u32,
    pub speed: f32,
    pub direction: Vec2,
    pub spawn: (f32, f32),
}