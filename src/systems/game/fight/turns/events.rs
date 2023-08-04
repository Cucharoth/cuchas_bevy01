use bevy::prelude::*;

#[derive(Event)]
pub struct PlayerDamageEvent(pub f32);

#[derive(Event)]
pub struct EnemyDamageEvent(pub f32);