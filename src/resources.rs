use crate::prelude::*;

#[derive(Resource)]
pub struct AfterEnemyCollisionTimer {
    pub timer: Timer,
}

impl Default for AfterEnemyCollisionTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}