use std::ops::Range;
use bevy::prelude::*;
use rand::Rng;

use crate::spaceship::movement::{Acceleration, MovingObjectBundle, Velocity};
use crate::asset_loader::SceneAssets;

use super::collision_detection::Collider;
const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct AsteroidsPlugin;
impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(Update, (spawn_asteroids, rotate_asteroid_z, handle_asteroid_collisions));
    }
}

fn spawn_asteroids(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.finished() {
        return;
    }
    let mut rng = rand::thread_rng();
    let transition = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.,
        rng.gen_range(SPAWN_RANGE_Z),
    );
    let mut random_unit_vector = 
        || Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0)).normalize_or_zero();
    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(velocity),
            acceleration: Acceleration::new(acceleration),
            model: SceneBundle {
                scene: scene_assets.asteroids.clone(),
                transform: Transform::from_translation(transition),
                ..default()
            },
            collider: Collider::new(2.5)
        },
        Asteroid,
    ));
}

fn rotate_asteroid_z(
    mut query: Query<&mut Transform, With<Asteroid>>,
    time: Res<Time>
) {
    for mut transform in query.iter_mut() {
        transform.rotate_z(2.5 * time.delta_seconds());
    }
}

fn handle_asteroid_collisions(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<Asteroid>>
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            if query.get(collided_entity).is_ok() {
                continue;
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}