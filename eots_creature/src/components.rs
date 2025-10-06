use bevy::prelude::*;
use eots_constants::creature::CreatureId;

#[derive(Reflect, Component, Debug, Clone, Copy)]
#[require(Transform)]
pub struct Creature(pub CreatureId);
    
#[derive(Reflect, Component, Debug, Clone, Copy)]
pub struct CreatureStats {
    // Health
    pub health: u32,
    pub max_health: u32,

    // Attack
    pub attack: u32,

    // Speed
    pub move_speed: f32,
    pub attack_speed: f32,

    // Range
    pub attack_range: f32,
    pub vision_range: f32,
}

impl From<&CreatureId> for CreatureStats {
    fn from(creature_id: &CreatureId) -> Self {
        let creature_data = creature_id.data();

        Self {
            health: creature_data.health,
            max_health: creature_data.health,
            attack: creature_data.attack,
            move_speed: creature_data.move_speed,
            attack_speed: creature_data.attack_speed,
            attack_range: creature_data.attack_range,
            vision_range: creature_data.vision_range,
        }
    }
}
