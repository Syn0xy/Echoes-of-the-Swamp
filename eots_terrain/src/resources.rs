use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Resource, Default, Debug, Clone)]
pub struct EndlessTerrain {
    pub chunks: HashMap<IVec2, Entity>,
    pub visible_chunks: Vec<IVec2>,
}
