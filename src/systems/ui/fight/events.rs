use bevy::prelude::*;

#[derive(Event)]
pub struct ReFocusButtonEvent(pub Entity);

#[derive(Event)]
pub struct HidePlayerSkillList{
    pub entity: Entity
}