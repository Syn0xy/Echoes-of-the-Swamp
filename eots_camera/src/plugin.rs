use bevy::{
    input::mouse::{MouseButtonInput, MouseWheel},
    prelude::*,
};
use eots_constants::camera::{
    CAMERA_ZOOM_SPEED, DEFAULT_CAMERA_PROJECTION_SCALE, MAX_CAMERA_SCALE, MIN_CAMERA_SCALE,
};

use crate::SmoothCameraTarget;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SmoothCameraTarget>()
            .add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                (
                    update_camera_zoom,
                    update_camera_reset_zoom,
                    update_smooth_camera,
                )
                    .chain(),
            );
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: DEFAULT_CAMERA_PROJECTION_SCALE,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn update_camera_zoom(
    mut rmessage_mouse_wheel: MessageReader<MouseWheel>,
    mut camera_q: Query<&mut Projection, With<Camera>>,
) {
    let Ok(mut projection) = camera_q.single_mut() else {
        return;
    };

    let Projection::Orthographic(orthographic_projection) = projection.as_mut() else {
        return;
    };

    for mouse_wheel in rmessage_mouse_wheel.read() {
        orthographic_projection.scale = (orthographic_projection.scale
            - mouse_wheel.y * CAMERA_ZOOM_SPEED)
            .clamp(MIN_CAMERA_SCALE, MAX_CAMERA_SCALE);
    }
}

fn update_camera_reset_zoom(
    mut rmessage_mouse_input: MessageReader<MouseButtonInput>,
    mut camera_q: Query<&mut Projection, With<Camera>>,
) {
    let Ok(mut projection) = camera_q.single_mut() else {
        return;
    };

    let Projection::Orthographic(orthographic_projection) = projection.as_mut() else {
        return;
    };

    for mouse_input in rmessage_mouse_input.read() {
        if mouse_input.button == MouseButton::Middle {
            orthographic_projection.scale = DEFAULT_CAMERA_PROJECTION_SCALE;
        }
    }
}

fn update_smooth_camera(
    mut camera_transform: Query<&mut Transform, With<Camera>>,
    target_transform: Query<(&Transform, &SmoothCameraTarget), Without<Camera>>,
    time: Res<Time>,
) {
    let Ok((target_transform, smooth_camera)) = target_transform.single() else {
        return;
    };

    let Ok(mut camera_transform) = camera_transform.single_mut() else {
        return;
    };

    camera_transform.translation.smooth_nudge(
        &target_transform.translation,
        smooth_camera.decay_rate,
        time.delta_secs(),
    );
}
