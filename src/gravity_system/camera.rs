use bevy::{input::mouse::MouseWheel, prelude::*};

const CAMERA_DISTANCE: f32 = 1020.0;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, handle_camera_move_opration)
            .add_systems(PostUpdate, handle_camera_zoom_opration);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}

fn handle_camera_move_opration(
    mut query: Query<&mut Transform, With<Camera3d>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    if keyboard_input.pressed(KeyCode::KeyW) {
        query.single_mut().translation += Vec3::Z * 200.0 * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        query.single_mut().translation -= Vec3::Z * 200.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        query.single_mut().translation += Vec3::X * 200.0 * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        query.single_mut().translation -= Vec3::X * 200.0 * time.delta_seconds();
    }
}

fn handle_camera_zoom_opration(
    mut query: Query<&mut Transform, With<Camera3d>>,
    mut wheel_reader: EventReader<MouseWheel>
) {
    for event in wheel_reader.read() {
        query.single_mut().translation -= Vec3::Y * event.y * 200.0;
    }
}