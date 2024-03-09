use crate::prelude::*;
use bevy::prelude::*;
pub struct EnemyPlugin;

#[derive(Component, Reflect)]
pub struct Enemy {
    pub base_experience_reward: i32,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {}
}
