use crate::prelude::*;

#[derive(Resource)]
pub struct PlayerSprite{
    pub sprite: Handle<Image>
}

#[derive(Resource)]
pub struct IntroTime{
    pub timer: Timer
}

impl Default for IntroTime {
    fn default() -> Self {
        Self { timer: Timer::from_seconds(1.0, TimerMode::Once)}
    }
}

#[derive(Resource)]
pub struct HalfIntroTime{
    pub timer: Timer
}

impl Default for HalfIntroTime {
    fn default() -> Self {
        Self { timer: Timer::from_seconds(0.5, TimerMode::Once)}
    }
}

