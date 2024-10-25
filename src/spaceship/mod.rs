use bevy::app::Plugin;

use asteroids::AsteroidsPlugin;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;
use collision_detection::CollisionDetectionPlugin;
use despawn::DespawnPlugin;
mod asteroids;
mod camera;
mod debug;
mod movement;
mod spaceship;
mod collision_detection;
mod despawn;
pub struct SpaceshipSystemPlugin;

impl Plugin for SpaceshipSystemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
          .add_plugins(SpaceshipPlugin)
          .add_plugins(AsteroidsPlugin)
          .add_plugins(CameraPlugin)
          .add_plugins(MovementPlugin)
          .add_plugins(CollisionDetectionPlugin)
          .add_plugins(DespawnPlugin)
          .add_plugins(DebugPlugin);
    }
}