use bevy::prelude::*;

use crate::prelude::fight::components::{PlayerSkill, Debuff};

#[derive(Event, Debug)]
pub struct PlayerDamageEvent{
    pub damage: f32,
    pub debuff: Option<Debuff>,
    pub debuff_duration: Option<f32>
}

#[derive(Debug)]
pub enum DamageType {
    Melee(f32),
    Skill(PlayerSkill)
}

#[derive(Event)]
pub struct EnemyDamageEvent(pub f32);