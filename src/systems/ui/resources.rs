use bevy::prelude::*;


#[derive(Resource)]
pub struct ButtonFocusChangeAudio{
    pub audio: Handle<AudioSource>
}

#[derive(Resource)]
pub struct ButtonInAudio{
    pub audio: Handle<AudioSource>
}

#[derive(Resource)]
pub struct ButtonOutAudio{
    pub audio: Handle<AudioSource>
}

