use std::borrow::BorrowMut;

use bevy::prelude::*;

use super::GravityStatusUpdateSet;


#[derive(Component)]
pub struct MotionComp {
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub self_rotation: Quat,
}
impl Default for MotionComp {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            self_rotation: Quat::from_rng(rand::thread_rng().borrow_mut())
        }
    }
}

pub struct MotionPlugin;
impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(FixedUpdate,
                (velocity_update).chain().in_set(GravityStatusUpdateSet::VelocityUpdate))
            .add_systems(FixedUpdate,
                (position_update).chain().in_set(GravityStatusUpdateSet::PositionUpdate));
    }
}

fn velocity_update(mut query: Query<&mut MotionComp>, time: Res<Time>) {
    for mut motion in query.iter_mut() {
        let acceleration = motion.acceleration.clone();
        if acceleration.x.is_nan() { continue; }
        motion.velocity += acceleration * time.delta_seconds();
    }
}

fn position_update(mut query: Query<(&mut Transform, &mut MotionComp)>, time: Res<Time>) {
    for (mut transform, motion) in query.iter_mut() {
        transform.translation += motion.velocity * time.delta_seconds();
    }
}
