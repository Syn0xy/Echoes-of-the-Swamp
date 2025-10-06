use bevy::prelude::*;

use crate::Perception;

const COLOR_YELLOW: Color = Color::linear_rgb(1.0, 0.5, 0.0);

pub fn debug_perception(mut gizmos: Gizmos, perception_q: Query<(&GlobalTransform, &Perception)>) {
    for (transform, perception) in &perception_q {
        gizmos.circle_2d(
            transform.translation().truncate(),
            perception.vision_range,
            COLOR_YELLOW,
        );
    }
}
