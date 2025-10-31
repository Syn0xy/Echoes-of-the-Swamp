use bevy::prelude::*;

use crate::{SegmentDescription, SegmentState, SegmentType};

const ANGLE_LINE_COLOR: Color = Color::linear_rgb(1.0, 1.0, 1.0);

const fn get_segment_type_color(segment_type: &SegmentType) -> Color {
    match segment_type {
        SegmentType::Head => Color::linear_rgb(1.0, 0.0, 0.0), // RED
        SegmentType::Body => Color::linear_rgb(0.0, 0.0, 1.0), // BLUE
        SegmentType::Tail => Color::linear_rgb(0.0, 1.0, 0.0), // GREEN
        SegmentType::Joint => Color::linear_rgb(1.0, 1.0, 0.0), // ORANGE
        SegmentType::Foot { .. } => Color::linear_rgb(1.0, 0.5, 0.0), // YELLOW
    }
}

pub(crate) fn draw_segment_gizmos(
    mut gizmos: Gizmos,
    segment_q: Query<(&GlobalTransform, &SegmentDescription, &SegmentState)>,
) {
    for (transform, desc, state) in &segment_q {
        let position = transform.translation().truncate();

        gizmos.circle_2d(
            position,
            desc.radius,
            get_segment_type_color(&desc.segment_type),
        );

        gizmos.line_2d(
            position,
            position + Vec2::from_angle(state.angle) * desc.length_offset,
            ANGLE_LINE_COLOR,
        );
    }
}
