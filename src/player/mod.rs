use crate::prelude::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component, Reflect)]
pub struct Player {
    pub level: i32,
    pub experience: i32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {}
}
