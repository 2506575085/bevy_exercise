use bevy::prelude::*;
pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_ship_info);
    }
}


fn print_ship_info(
  query: Query<(Entity, &Transform)>
) {
  for (_entity, _transform) in query.iter() {
      // println!("{:?}:{:?}", entity, transform);
  }
}
