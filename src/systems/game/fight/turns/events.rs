use bevy::prelude::*;

#[derive(Event)]
pub struct PlayerDamageEvent(pub u32);

#[derive(Event)]
pub struct EnemyDamageEvent(pub u32);