use bevy::prelude::*;

use super::GravityStatusUpdateSet;

#[derive(Component)]
pub struct CollisionDetection {
    pub radius: f32
}

#[derive(Event, Debug)]
pub struct CollisionDetectionEvent {
    pub entity: Entity,
    pub other_entity: Entity,
}

pub struct CollisionDetectionPlugin;
impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionDetectionEvent>();
        app.add_systems(FixedUpdate, collision_detection_system.chain().in_set(GravityStatusUpdateSet::CollisionDetection));
    }
}

fn collision_detection_system(
    mut events_writer: EventWriter<CollisionDetectionEvent>,
    query: Query<(Entity, &GlobalTransform, &CollisionDetection), With<CollisionDetection>>,
) {
    for (entity, transform, collection) in query.iter() {
        for (other_entity, other_transform, other_collection) in query.iter() {
            if entity == other_entity { continue; }
            if transform.translation().distance(other_transform.translation()) <= collection.radius + other_collection.radius {
                events_writer.send(CollisionDetectionEvent {
                    entity,
                    other_entity,
                });
            }
        }
    }
}