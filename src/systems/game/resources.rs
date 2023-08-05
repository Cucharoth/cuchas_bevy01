use crate::prelude::*;

#[derive(Resource)]
pub struct EnemyEntityCollisioned {
    pub(crate) entity: Entity,
}

#[derive(Resource)]
pub struct PlayerEntity {
    pub(crate) entity: Entity,
}

#[derive(Resource, Debug)]
pub struct PlayerStatus {
    pub health: f32,
    pub damage: f32,
    pub speed: f32,
    pub mana: f32,
    pub transform: Transform,
    pub bad_luck_protection: f64 // rng uses f64
}

impl Default for PlayerStatus {
    fn default() -> Self {
        Self {
            health: 100.0,
            damage: 10.0,
            speed: 100.0,
            mana: 100.0,
            transform: Transform::default(),
            bad_luck_protection: 0.
        }
    }
}
