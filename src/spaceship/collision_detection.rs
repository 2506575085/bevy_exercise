use bevy::{prelude::*, utils::HashMap};

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Collider {
            radius,
            colliding_entities: Vec::new(),
        }
    }
}

pub struct CollisionDetectionPlugin;
impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
    }
}

fn collision_detection(
    mut query: Query<(Entity, &mut GlobalTransform, &mut Collider)>
) {
    let mut colliding_entities = HashMap::new();
    for (entity, transform, collider) in query.iter() {
        for (entity_other, transform_other, collider_other) in query.iter() {
            if entity == entity_other {
                continue;
            }
            let distance = transform.translation().distance(transform_other.translation());
            if distance < collider.radius + collider_other.radius {
                colliding_entities
                    .entry(entity)
                    .or_insert_with(Vec::new)
                    .push(entity_other);
            }
        }
    }
    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend(collisions.iter().copied());
        }
    }
}