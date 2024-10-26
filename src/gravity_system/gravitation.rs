use bevy::{prelude::*, utils::HashMap};
use super::{motion::MotionComp, GravityStatusUpdateSet};

#[derive(Component)]
pub struct GravitationComp {
    pub mass: f32,
}
impl GravitationComp {
    pub fn new(mass: f32) -> Self {
        Self { mass }
    }
}

pub struct GravitationPlugin;
impl Plugin for GravitationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate,
            acceleration_update.chain().in_set(GravityStatusUpdateSet::AccelerationUpdate));
    }
}

fn acceleration_update(
    mut query: Query<(Entity, &GlobalTransform, &GravitationComp, &mut MotionComp)>
) {
    let mut new_accelerations_map: HashMap<Entity, Vec3> = HashMap::new();
    for (entity, transform, _, _) in query.iter() {
        let mut acclerations = Vec::new();
        for (other_entity ,other_transform, other_gravitation, _) in query.iter() {
            if entity == other_entity { continue; }
            let distance = transform.translation().distance(other_transform.translation());
            let acceleration = 6.67 * 10.0_f32.powi(-11) * other_gravitation.mass / distance.powi(2);
            let dir = (other_transform.translation() - transform.translation()).normalize();
            acclerations.push(dir*acceleration);
        }
        new_accelerations_map.insert(entity, acclerations.iter().sum::<Vec3>());
    }
    for (entity, _, _, mut motion) in query.iter_mut() {
        motion.acceleration = *new_accelerations_map.get(&entity).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
  #[test]
  fn it_works() {
    let position1 = Vec3::ZERO;
    let position2 = Vec3::new(2.0, 0.0, 0.0);
    println!("{:?}", (position2 - position1).normalize());
  }
}
