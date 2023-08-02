use crate::prelude::*;

#[derive(Resource)]
pub struct EnemyEntity {
    pub(crate) entity: Entity,
}

#[derive(Resource)]
pub struct PlayerEntity {
    pub(crate) entity: Entity,
}

#[derive(Resource, Debug)]
pub struct PlayerStatus {
    pub health: u32,
    pub damage: u32,
    pub speed: u32,
    pub mana: u32,
}

impl Default for PlayerStatus {
    fn default() -> Self {
        Self {
            health: 100,
            damage: 10,
            speed: 100,
            mana: 100,
        }
    }
}
