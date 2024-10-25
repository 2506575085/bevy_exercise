use bevy::prelude::*;
use planet::PlanetPlugin;
use gravitation::GravitationPlugin;
use motion::MotionPlugin;
use camera::CameraPlugin;
use running_state::{RunningState, RunningStatePlugin};
use debugger::DebuggerPlugin;
use collision_detection::CollisionDetectionPlugin;

mod gravitation;
mod motion;
mod planet;
mod camera;
mod running_state;
mod debugger;
mod collision_detection;

pub struct GravitySystemPlugin;
impl Plugin for GravitySystemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .configure_sets(
                FixedUpdate,
                (
                    GravityStatusUpdateSet::AccelerationUpdate,
                    GravityStatusUpdateSet::VelocityUpdate,
                    GravityStatusUpdateSet::CollisionDetection,
                    GravityStatusUpdateSet::PositionUpdate,
                ).chain()
                .run_if(in_state(RunningState::Running))
            )
            .add_plugins(RunningStatePlugin)
            .add_plugins(CollisionDetectionPlugin)
            .add_plugins(DebuggerPlugin)
            .add_plugins(PlanetPlugin)
            .add_plugins(MotionPlugin)
            .add_plugins(GravitationPlugin)
            .add_plugins(CameraPlugin);
    }
}


#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GravityStatusUpdateSet {
    AccelerationUpdate,
    VelocityUpdate,
    CollisionDetection,
    PositionUpdate,
}