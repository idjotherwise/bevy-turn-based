use crate::prelude::*;
use bevy::prelude::*;

pub struct CharacterPlugin;

#[derive(Component, Reflect)]
pub struct CharacterBundle {}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {}
}
