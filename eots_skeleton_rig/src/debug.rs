use bevy::prelude::*;

use crate::{SegmentDescription, SegmentState, descriptions::SegmentType, utils};

const fn get_segment_type_color(segment_type: &SegmentType) -> Color {
    match segment_type {
        SegmentType::Head => Color::linear_rgb(1.0, 0.0, 0.0), // RED
        SegmentType::Body => Color::linear_rgb(0.0, 0.0, 1.0), // BLUE
        SegmentType::Tail => Color::linear_rgb(0.0, 1.0, 0.0), // GREEN
        SegmentType::Joint => Color::linear_rgb(1.0, 1.0, 0.0), // ORANGE
        SegmentType::Foot { .. } => Color::linear_rgb(1.0, 0.5, 0.0), // YELLOW
    }
}

pub fn draw_segment_gizmos(
    mut gizmos: Gizmos,
    segment_q: Query<(Entity, &GlobalTransform, &SegmentDescription)>,
    segment_state: Query<&SegmentState>,
) {
    for (entity, transform, segment_description) in &segment_q {
        let position = transform.translation().truncate();

        gizmos.circle_2d(
            position,
            segment_description.radius,
            get_segment_type_color(&segment_description.segment_type),
        );

        if let Ok(SegmentState { angle, .. }) = segment_state.get(entity) {
            gizmos.line_2d(
                position,
                position + utils::vec2_from_angle_deg(*angle) * segment_description.length_offset,
                Color::linear_rgb(1.00, 1.0, 1.0),
            );
        }
    }
}
