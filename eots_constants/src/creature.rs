use bevy::prelude::*;
use eots_skeleton_rig::descriptions::SkeletonRigDescription;

mod fly;
mod frog;
mod salamander;
mod scolopendra;
mod spider;
mod toad;
mod worm;

pub const ENEMY_GROUP_LAYER: u8 = 1;
pub const PLAYER_GROUP_LAYER: u8 = 2;

#[derive(Reflect, Debug, Clone, Copy)]
pub enum CreatureId {
    Salamander,
    Fly,
    Worm,
    Toad,
    Frog,
    Scolopendra,
    Spider,
}

#[derive(Reflect, Debug, Clone, Copy)]
pub struct CreatureData {
    // Description
    pub name: &'static str,

    // Stats
    pub health: u32,
    pub attack: u32,

    // Speed
    pub move_speed: f32,
    pub attack_speed: f32,

    // Range
    pub attack_range: f32,
    pub vision_range: f32,
}

impl CreatureId {
    pub const fn data(&self) -> &CreatureData {
        match self {
            CreatureId::Salamander => &salamander::DATA,
            CreatureId::Fly => &fly::DATA,
            CreatureId::Worm => &worm::DATA,
            CreatureId::Toad => &toad::DATA,
            CreatureId::Frog => &frog::DATA,
            CreatureId::Scolopendra => &scolopendra::DATA,
            CreatureId::Spider => &spider::DATA,
        }
    }

    pub const fn skeleton_rig(&self) -> &SkeletonRigDescription {
        match self {
            CreatureId::Salamander => &salamander::SKELETON_RIG,
            CreatureId::Fly => &fly::SKELETON_RIG,
            CreatureId::Worm => &worm::SKELETON_RIG,
            CreatureId::Toad => &toad::SKELETON_RIG,
            CreatureId::Frog => &frog::SKELETON_RIG,
            CreatureId::Scolopendra => &scolopendra::SKELETON_RIG,
            CreatureId::Spider => &spider::SKELETON_RIG,
        }
    }
}
