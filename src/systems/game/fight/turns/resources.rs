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
pub struct FightWinTimer{
    pub timer: Timer
}

impl Default for FightWinTimer {
    fn default() -> Self {
        Self { timer: Timer::from_seconds(3.0, TimerMode::Once) }
    }
}

#[derive(Resource)]
pub struct FightLostTimer{
    pub timer: Timer
}

impl Default for FightLostTimer {
    fn default() -> Self {
        Self { timer: Timer::from_seconds(4.0, TimerMode::Once) }
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