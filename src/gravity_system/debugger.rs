use bevy::prelude::*;

use super::{
    motion::MotionComp,
    planet::SmallPlanet,
};
#[derive(Resource, Debug)]
struct DebugTimer(Timer);

pub struct DebuggerPlugin;
impl Plugin for DebuggerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(
            FixedUpdate,
            watch_small_planet
        );
    }
}

fn watch_small_planet(
    time: Res<Time>,
    mut timer: ResMut<DebugTimer>,
    query: Query<(&Transform, &MotionComp), With<SmallPlanet>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        let (transform, motion) = query.single();
        println!(
            "******************\nsmall planet Position: {:?}\nVelocity: {:?}\nAcceleration: {:?}\n******************",
            transform.translation,
            motion.velocity,
            motion.acceleration
        );
    }
}
