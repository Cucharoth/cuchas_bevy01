use bevy::prelude::*;

#[derive(Resource)]
pub struct DamageHappeningTimer{
    pub timer: Timer
}

impl Default for DamageHappeningTimer {
    fn default() -> Self {
        Self { timer: Timer::from_seconds(1.0, TimerMode::Once) }
    }
}

#[derive(Resource)]
pub struct PlayerActiveLastTurn(pub bool);

impl Default for PlayerActiveLastTurn {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Resource)]
pub struct PlayerIsDefending(pub bool);

impl Default for PlayerIsDefending {
    fn default() -> Self {
        Self(false)
    }
}