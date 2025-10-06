use perlin_noise::{NoiseDescriptor, NormalizeMode};

use crate::{
    biome::{BiomeCreatures, BiomeData, TerrainDescriptor, TerrainHandler},
    creature::CreatureId,
};

pub(super) const DATA: BiomeData = BiomeData {
    name: "Swamp",
    creatures: BiomeCreatures {
        level_1: &[CreatureId::Worm, CreatureId::Fly],
        _level_2: &[CreatureId::Frog],
        _level_3: &[CreatureId::Toad],
        _boss: &[CreatureId::Scolopendra, CreatureId::Spider],
    },
    terrain_descriptors: &[
        TerrainDescriptor {
            label: "map",
            noise_descriptor: NoiseDescriptor {
                seed: 2,
                scale: 80.0,
                octaves: 2,
                persistance: 0.5,
                lacunarity: 2.0,
                normalize_mode: NormalizeMode::Global,
            },
            _handler: TerrainHandler {},
        },
        TerrainDescriptor {
            label: "tree",
            noise_descriptor: NoiseDescriptor {
                seed: 2,
                scale: 1.0,
                octaves: 1,
                persistance: 1.0,
                lacunarity: 1.0,
                normalize_mode: NormalizeMode::Local,
            },
            _handler: TerrainHandler {},
        },
    ],
};
