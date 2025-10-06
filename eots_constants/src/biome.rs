use perlin_noise::NoiseDescriptor;
use strum_macros::EnumIter;

use crate::creature::CreatureId;

mod abyss;
mod caverns;
mod crystal;
mod desert;
mod jungle;
mod mushroom_fields;
mod ocean;
mod plains;
mod swamp;
mod tundra;
mod underworld;
mod volcano;

#[derive(EnumIter, Debug, Clone, Copy)]
pub enum BiomeId {
    Swamp,
    Plains,
    Desert,
    Tundra,
    Caverns,
    Jungle,
    Ocean,
    Abyss,
    Volcano,
    Underworld,
    MushroomFields,
    Crystal,
}

#[derive(Debug, Clone, Copy)]
pub struct BiomeData {
    pub name: &'static str,
    pub creatures: BiomeCreatures,
    pub terrain_descriptors: &'static [TerrainDescriptor],
}

#[derive(Debug, Clone, Copy)]
pub struct BiomeCreatures {
    pub level_1: &'static [CreatureId],
    pub _level_2: &'static [CreatureId],
    pub _level_3: &'static [CreatureId],
    pub _boss: &'static [CreatureId],
}

#[derive(Debug, Clone, Copy)]
pub struct TerrainDescriptor {
    pub label: &'static str,
    pub noise_descriptor: NoiseDescriptor,
    pub _handler: TerrainHandler,
}

#[derive(Debug, Clone, Copy)]
pub struct TerrainHandler {}

impl BiomeId {
    pub const fn data(&self) -> &'static BiomeData {
        match self {
            BiomeId::Swamp => &swamp::DATA,
            BiomeId::Plains => &plains::DATA,
            BiomeId::Desert => &desert::DATA,
            BiomeId::Tundra => &tundra::DATA,
            BiomeId::Caverns => &caverns::DATA,
            BiomeId::Jungle => &jungle::DATA,
            BiomeId::Ocean => &ocean::DATA,
            BiomeId::Abyss => &abyss::DATA,
            BiomeId::Volcano => &volcano::DATA,
            BiomeId::Underworld => &underworld::DATA,
            BiomeId::MushroomFields => &mushroom_fields::DATA,
            BiomeId::Crystal => &crystal::DATA,
        }
    }
}
