use bevy::{input::mouse::MouseWheel, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        // app.add_plugins(bevy_blendy_cameras::BlendyCamerasPlugin);
        app.add_systems(Update, handle_camera_move_operation);
        //   app.add_systems(Update, handle_camera_zoom_operation);
    }
}

#[derive(Component)]
struct WaveFunctionSystemCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(310., 300., 0.),
                ..default()
            },
            ..default()
        },
        WaveFunctionSystemCamera,
    ));
}

fn handle_camera_move_operation(
    mut query: Query<&mut Transform, With<WaveFunctionSystemCamera>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if keyboard_input.pressed(KeyCode::KeyW) {
        query.single_mut().translation += Vec3::Y * 200.0 * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        query.single_mut().translation -= Vec3::Y * 200.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        query.single_mut().translation -= Vec3::X * 200.0 * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        query.single_mut().translation += Vec3::X * 200.0 * time.delta_seconds();
    }
}

// fn handle_camera_zoom_operation(
//     mut query: Query<&mut Projection, With<WaveFunctionSystemCamera>>,
//     // mut wheel_reader: EventReader<MouseWheel>
// ) {
//     // for event in wheel_reader.read() {
//         match query.single_mut().into_inner() {
//             Projection::Orthographic(projection) => {
//                 if (projection.scale - 0.15).abs() <= f32::EPSILON {
//                     projection.scale = 0.15;
//                 } else {
//                     projection.scale = 0.15;
//                 }
//             },
//             Projection::Perspective(projection) => {
//                 if (projection.fov - 0.785).abs() <= f32::EPSILON {
//                     projection.fov = std::f32::consts::PI / 2.;
//                 }                 else {
//                     projection.fov = 0.785;
//                 }
//             }
//         }
//     // }
// }
