use std::fs;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::asset_loader::SceneAssets;
use crate::gravity_system::gravitation::GravitationComp;
use crate::gravity_system::motion::MotionComp;

use super::collision_detection::{CollisionDetection, CollisionDetectionEvent};
use super::running_state::{ResetEvent, RunningState};
use super::GravityStatusUpdateSet;

#[derive(Bundle)]
struct Planet {
    gravitation: GravitationComp,
    motion: MotionComp,
    model: SceneBundle,
    collision_detection: CollisionDetection,
}

impl Default for Planet {
    fn default() -> Self {
        Self {
            gravitation: GravitationComp::new(0.0),
            motion: MotionComp::default(),
            model: SceneBundle::default(),
            collision_detection: CollisionDetection { radius: 10.0 }
        }
    }
}

impl Planet {
    fn new(mass: f32, transform: Transform, velocity: Vec3, asset_model:Handle<Scene>, radius: f32) -> Self {
        Self {
            model: SceneBundle {
                transform,
                scene: asset_model,
                ..default()
            },
            motion: MotionComp {
                velocity,
                ..default()
            },
            gravitation: GravitationComp::new(mass),
            collision_detection: CollisionDetection { radius },
            ..default()
        }
    }
}

#[derive(Component)]
pub struct SmallPlanet;
#[derive(Component)]
pub struct FixedStar;

pub struct PlanetPlugin;
impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_planets);
        app.add_systems(Update, self_rotate);
        app.add_systems(Update,
            (clear_planets, spawn_planets).chain().run_if(on_event::<ResetEvent>()));
        app.add_systems(FixedUpdate,
            handle_planet_collision.chain()
                        .after(GravityStatusUpdateSet::CollisionDetection)
                        .run_if(on_event::<CollisionDetectionEvent>()));
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Vec3Json {
    x: f32,
    y: f32,
    z: f32,
}
#[derive(Serialize, Deserialize ,Debug)]
struct PlanetJson {
    mass: f32,
    position: Vec3Json,
    velocity: Vec3Json,
    radius: f32
}
fn get_planets_from_json(path: &'static str) -> Option<Vec<PlanetJson>> {
    let fixed_stars = fs::read_to_string(path).ok();
    if let Some(fixed_stars) = fixed_stars {
        serde_json::from_str::<Vec<PlanetJson>>(&fixed_stars).ok()
    } else {
        None
    }
}

fn spawn_planets(mut commands: Commands, asset_model: Res<SceneAssets>) {
    let fixed_stars = get_planets_from_json("assets/json/fixed_stars.json");
    if let Some(fixed_stars) = fixed_stars {
        for PlanetJson { mass, position, velocity, radius } in fixed_stars {
            commands.spawn((Planet::new(
                mass,
                Transform {
                    translation: Vec3::new(position.x, position.y, position.z),
                    scale: Vec3::splat(radius / 2.0),
                    ..default()
                },
                Vec3::new(velocity.x, velocity.y, velocity.z),
                asset_model.asteroids.clone(),
                radius
            ), FixedStar));
        }
    }
    let planets = get_planets_from_json("assets/json/planets.json");
    if let Some(planets) = planets {
        for PlanetJson { mass, position, velocity, radius } in planets {
            commands.spawn((
                Planet::new(
                    mass,
                    Transform {
                        translation: Vec3::new(position.x, position.y, position.z),
                        scale: Vec3::splat(radius / 2.0),
                        ..default()
                    },
                    Vec3::new(velocity.x, velocity.y, velocity.z),
                    asset_model.planet.clone(),
                    radius
                ),
                SmallPlanet
            ));
        }
    }
}

fn clear_planets(
    mut commands: Commands,
    planets: Query<Entity, Or<(With<SmallPlanet>, With<FixedStar>)>>
) {
    for entity in planets.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_planet_collision(
    mut collision_events: EventReader<CollisionDetectionEvent>,
    query: Query<(Entity, &mut Transform), Or<(With<SmallPlanet>, With<FixedStar>)>>,
    mut running_state: ResMut<NextState<RunningState>>
) {
    for event in collision_events.read() {
        let t1 = query.get(event.entity).unwrap().1;
        let t2 = query.get(event.other_entity).unwrap().1;
        println!("Collision detected!{:#?}{:#?}", t1, t2);
    }
    running_state.set(RunningState::End);
}

fn self_rotate(
    mut query: Query<(&mut Transform, &MotionComp), Or<(With<SmallPlanet>, With<FixedStar>)>>,
    time: Res<Time>
) {
    for (mut transform, motion) in query.iter_mut() {
        transform.rotate_y(motion.self_rotation.y * time.delta_seconds());
        transform.rotate_x(motion.self_rotation.x * time.delta_seconds());
        transform.rotate_z(motion.self_rotation.z * time.delta_seconds());
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    let planets = get_planets_from_json("assets/json/fixed_stars.json");
    println!("planets: {:?}", planets)
  }
}