use bevy::prelude::*;

use crate::PositionStrategy;

#[derive(Resource, Default)]
pub struct SpawnerManager {
    pub(crate) spawners: Vec<(Timer, SpawnerConfig)>,
}

pub struct SpawnerConfig {
    pub position_strategy: Box<dyn PositionStrategy>,
    pub origin: Vec2,
    pub radius: f32,
    pub spawn_action: Box<dyn Fn(&mut Commands, Vec2) + Send + Sync>,
}

impl SpawnerManager {
    pub fn add(&mut self, timer: Timer, config: SpawnerConfig) {
        self.spawners.push((timer, config));
    }

    pub fn clear(&mut self) {
        self.spawners.clear();
    }
}

impl SpawnerConfig {
    pub fn generate_position(&self) -> Vec2 {
        self.position_strategy.generate_position(&self.origin, self.radius)
    }
}