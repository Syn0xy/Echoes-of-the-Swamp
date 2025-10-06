use bevy::prelude::*;
use eots_constants::biome::{BiomeData, BiomeId};
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

#[derive(Resource, Default, Debug, Clone)]
pub struct SessionData {
    pub biome_pool: Vec<BiomeId>,
    pub current_biome: Option<BiomeId>,
}

impl SessionData {
    pub const fn set_current_biome(&mut self, biome: BiomeId) {
        self.current_biome = Some(biome);
    }

    pub const fn current_biome_data(&self) -> Option<&'static BiomeData> {
        match self.current_biome {
            Some(biome) => Some(biome.data()),
            None => None,
        }
    }

    pub fn reset_biome_pool(&mut self) -> &mut Vec<BiomeId> {
        self.biome_pool = BiomeId::iter().collect();
        self.biome_pool.shuffle(&mut rand::rng());
        &mut self.biome_pool
    }
}
