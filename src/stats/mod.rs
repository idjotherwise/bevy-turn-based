use crate::prelude::*;
use bevy::prelude::*;
pub struct StatsPlugin;

#[derive(Component, Reflect)]
pub struct Stats {
    pub health: i32,
    pub health_max: i32,
    pub attack: i32,
    pub defence: i32,
    pub crit_strike: i32,
}

impl Stats {
    fn default() -> Self {
        Self {
            health: 10,
            health_max: 10,
            attack: 3,
            defence: 1,
            crit_strike: 0,
        }
    }
}

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {}
}
