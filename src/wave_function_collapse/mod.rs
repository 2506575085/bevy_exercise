use std::fs;
use bevy::prelude::*;
use bevy::utils::HashSet;
use rand::seq::IteratorRandom;
use rand::Rng;
use core::{WaveConfig, WaveGrid, ConfigMap, GridPosition};

mod camera;
mod core;

static GRID_WIDTH: u32 = 50;
static GRID_HEIGHT: u32 = 50;

pub struct WaveFunctionCollapsePlugin;
impl Plugin for WaveFunctionCollapsePlugin {
    fn build(&self, app: &mut App) {
        let (wave_config, wave_grid) = get_init_resource();
        app.insert_resource(wave_config);
        app.insert_resource(wave_grid);
        app.add_systems(FixedUpdate, (wave_function_collapse, update_grid_rendering));
        app.add_plugins(camera::CameraPlugin);
    }
}


fn wave_function_collapse(
    mut wave_grid: ResMut<WaveGrid>,
    wave_config: Res<WaveConfig>,
) {
    while let Some(GridPosition {x, y}) = wave_grid.stack.pop() {
        let beside_impossible = wave_grid.grid_value.get_item(x, y).get_beside_impossible_value(&wave_config.0);
        if y != GRID_HEIGHT - 1 { wave_grid.update_grid(GridPosition::new(x, y + 1), &beside_impossible.top); }
        if x != GRID_WIDTH - 1 { wave_grid.update_grid(GridPosition::new(x + 1, y), &beside_impossible.right); }
        if y != 0 { wave_grid.update_grid(GridPosition::new(x, y - 1), &beside_impossible.bottom); }
        if x != 0 { wave_grid.update_grid(GridPosition::new(x - 1, y), &beside_impossible.left); }
    }
    let new_position = wave_grid.get_rand_not_collapse();
    if let Some(new_position) = new_position {
        let wave_grid_item = wave_grid.grid_value.get_item(new_position.x, new_position.y);
        let new_value = wave_grid_item.possible_value.read().unwrap().iter().choose(&mut rand::thread_rng()).cloned().unwrap();
        wave_grid.update_grid_item_value(new_position, new_value);
    }
}

fn update_grid_rendering(
    config: Res<WaveConfig>,
    mut wave_grid: ResMut<WaveGrid>,
    mut commands: Commands,
    asset_loader: Res<AssetServer>
) {
    for grid_row in wave_grid.grid_value.get_mut_value().iter_mut() {
        for grid_item in grid_row.iter_mut() {
            if grid_item.get_value().is_none() || grid_item.rendered {
                continue;
            }
            let (position, asset_path_str) = grid_item.get_model_bundle(&config.0);
            let scale = 0.1;
            if let Some(_) = grid_item.get_value() {
                commands.spawn(SpriteBundle {
                    texture: asset_loader.load(asset_path_str),
                    transform: Transform {
                        translation: Vec3::new(position.x as f32 * 128. * scale, position.y as f32 * 128. * scale,0.0),
                        scale: Vec3::new(scale, scale, 0.),
                        ..default()
                    },
                    ..default()
                });
                grid_item.rendered = true;
            }
        }
    }
}

fn get_init_resource() -> (WaveConfig, WaveGrid) {
    let (config, all_possible_value) = get_config();
    let random_value = all_possible_value.iter().choose(&mut rand::thread_rng()).unwrap().clone();
    let random_x = rand::thread_rng().gen_range(0..GRID_WIDTH);
    let random_y = rand::thread_rng().gen_range(0..GRID_HEIGHT);
    let mut wave_grid = WaveGrid::new(GRID_WIDTH as usize, GRID_HEIGHT as usize, all_possible_value);

    wave_grid.update_grid_item_value(GridPosition::new(random_x, random_y), random_value);
    println!("init:{:?}\nx:{:?},y:{:?}", random_value, random_x, random_y);
    (WaveConfig(config), wave_grid)
}

fn get_config() -> (ConfigMap, HashSet<u32>) {
    let json_str = fs::read_to_string("assets/json/wave_function_collapse_map.json").ok().unwrap();
    let config = serde_json::from_str::<ConfigMap>(&json_str).ok().unwrap();
    (
        config.clone(),
        config.iter().map(|(&code, _)| code).collect::<HashSet<u32>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::core::*;
      use super::*;
      #[test]
      fn it_works() {
            let a = GridItem::new(GridPosition::new(1, 1), HashSet::from([9,10,13,14]));
            println!("{:?}",a.get_beside_impossible_value(&get_init_resource().0.0));
            // println!("{:#?}", get_init_resource());
      }
}