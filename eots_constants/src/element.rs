use bevy::prelude::*;

#[derive(Reflect, Default, Debug, Clone, Copy)]
pub enum ElementType {
    #[default]
    Neutral,
    Water,
    Fire,
    Wind,
    Earth,
    Ice,
    Electric,
    Plant,
    Poison,
}
