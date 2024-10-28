use std::fs;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use rand::seq::IteratorRandom;
use rand::Rng;
use serde::Deserialize;
use std::sync::RwLock;

mod camera;

static GRID_WIDTH: usize = 50;
static GRID_HEIGHT: usize = 50;

pub struct WaveFunctionCollapsePlugin;
impl Plugin for WaveFunctionCollapsePlugin {
    fn build(&self, app: &mut App) {
        let (wave_config, wave_grid, wave_stack) = get_init_resource();
        app.insert_resource(wave_config);
        app.insert_resource(wave_grid);
        app.insert_resource(wave_stack);
        app.add_systems(FixedUpdate, (wave_function_collapse, update_grid_rendering));
        app.add_plugins(camera::CameraPlugin);
    }
}

#[derive(Component ,Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct GridPosition {
    x: u32,
    y: u32,
}
impl GridPosition {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
struct BesideImpossible {
    top: HashSet<u32>,
    right: HashSet<u32>,
    bottom: HashSet<u32>,
    left: HashSet<u32>,
}

#[derive(Debug, Deserialize, Clone)]
struct ModelConfigOptions {
    // code: u32,
    beside_impossible: BesideImpossible,
    asset_model: String
}

type ConfigMap = HashMap<u32, ModelConfigOptions>;

#[derive(Component, Debug)]
struct GridItem {
    value: Option<u32>,
    possible_value: RwLock<HashSet<u32>>,
    position: GridPosition,
    rendered: bool,
}

impl GridItem {
    fn new(position: GridPosition, possible_value: HashSet<u32>) -> Self {
        let possible_value = RwLock::new(possible_value);
        Self {
            value: None,
            possible_value,
            position,
            rendered: false
        }
    }
    fn get_model_bundle(&self, config: &ConfigMap) -> (&GridPosition, String) {
        (
            &self.position,
            self.get_config(config).asset_model.clone()
        )
    }
    // 必须value不为空才可调用
    fn get_config<'a>(&self, config: &'a ConfigMap) -> &'a ModelConfigOptions {
        config.get(&self.value.unwrap()).unwrap()
    }
    fn get_beside_impossible_value(&self, config: &ConfigMap) -> BesideImpossible {
        if let Some(_) = self.value {
            self.get_config(config).beside_impossible.clone()
        } else {
            let possible_config: Vec<BesideImpossible> = self.possible_value.read().unwrap()
                .iter().map(|possible_code| &config.get(possible_code).unwrap().beside_impossible).cloned().collect();

            let res = possible_config.iter().fold(possible_config[0].clone(), |acc, value| {
                let new_top = acc.top.intersection(&value.top).cloned().collect();
                let new_right = acc.right.intersection(&value.right).cloned().collect();
                let new_bottom = acc.bottom.intersection(&value.bottom).cloned().collect();
                let new_left = acc.left.intersection(&value.left).cloned().collect();
                BesideImpossible {
                    top: new_top,
                    right: new_right,
                    bottom: new_bottom,
                    left: new_left,
                }
            });
            res
        }
    }
}

#[derive(Resource, Debug)]
struct WaveGrid(Vec<Vec<GridItem>>);
impl WaveGrid {
    fn new(width: usize, height: usize, all_possible_value: HashSet<u32>) -> Self {
        let mut vector: Vec<Vec<GridItem>> = vec![];
        for row_index in 0..height {
            let mut row_vec = vec![];
            for col_index in 0..width {
                row_vec.push(GridItem::new(
                    GridPosition::new(col_index as u32, row_index as u32),
                    all_possible_value.clone()
                ));
            }
            vector.push(row_vec);
        }
        Self(vector)
    }
    fn get_rand_unrendered(&self) -> Option<GridPosition> {
        let mut unrendered = vec![];
        for row in self.0.iter() {
            for item in row {
                if item.value.is_none() {
                    unrendered.push(item.position.clone());
                }
            }
        }
        if unrendered.is_empty() {
            return None;
        }
        Some(*unrendered.iter().choose(&mut rand::thread_rng()).unwrap())
    }
}

#[derive(Resource, Debug)]
struct WaveStack(Vec<GridPosition>);
impl WaveStack {
    fn push(&mut self, position: GridPosition) {
        if self.0.contains(&position) {
            return;
        }
        self.0.push(position);
    }
}

#[derive(Resource, Debug)]
struct WaveConfig(ConfigMap);

fn wave_function_collapse(
    mut wave_grid: ResMut<WaveGrid>,
    mut wave_stack: ResMut<WaveStack>,
    wave_config: Res<WaveConfig>,
) {
    while let Some(GridPosition {x, y}) = wave_stack.0.pop() {
        let x_usize = x as usize;
        let y_usize = y as usize;
        let beside_impossible = wave_grid.0[y_usize][x_usize].get_beside_impossible_value(&wave_config.0);
        let need_update_info = [
            if y_usize != GRID_HEIGHT - 1 { Some((x_usize, y_usize + 1, &beside_impossible.top)) } else { None },
            if x_usize != GRID_WIDTH - 1 { Some((x_usize + 1, y_usize, &beside_impossible.right)) } else { None },
            if y_usize != 0 { Some((x_usize, y_usize - 1, &beside_impossible.bottom)) } else { None },
            if x_usize != 0 { Some((x_usize - 1, y_usize, &beside_impossible.left)) } else { None }
        ];
        for info in need_update_info {
            if let Some((x, y, impossible)) = info {
                update_grid(&mut wave_grid.0[y][x], impossible, &mut wave_stack);
            }
        }
    }
    let new_position = wave_grid.get_rand_unrendered();
    if let Some(new_position) = new_position {
        let wave_grid_item = &mut wave_grid.0[new_position.y as usize][new_position.x as usize];
        let new_value = wave_grid_item.possible_value.read().unwrap().iter().choose(&mut rand::thread_rng()).cloned().unwrap();
        update_grid_item_value(wave_grid_item, new_value, &mut wave_stack);
    }
}

fn update_grid(
    grid_item: &mut GridItem,
    beside_impossible: &HashSet<u32>,
    stack: &mut WaveStack
) {
    if grid_item.value.is_some() {
        return;
    }
    let old_len = grid_item.possible_value.read().unwrap().len();
    grid_item.possible_value.write().unwrap().retain(|value| !beside_impossible.contains(value));
    if old_len != grid_item.possible_value.read().unwrap().len() {
        stack.push(grid_item.position.clone());
    }
    if grid_item.possible_value.read().unwrap().len() == 1 {
        let value = grid_item.possible_value.read().unwrap().iter().next().cloned().unwrap();
        update_grid_item_value(grid_item, value, stack);
    }
}

fn update_grid_rendering(
    config: Res<WaveConfig>,
    mut wave_grid: ResMut<WaveGrid>,
    mut commands: Commands,
    asset_loader: Res<AssetServer>
) {
    for grid_row in wave_grid.0.iter_mut() {
        for grid_item in grid_row.iter_mut() {
            if grid_item.value.is_none() || grid_item.rendered {
                continue;
            }
            let (position, asset_path_str) = grid_item.get_model_bundle(&config.0);
            let scale = 0.1;
            if let Some(_) = grid_item.value {
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

fn update_grid_item_value(grid_item: &mut GridItem, value: u32, stack: &mut WaveStack) {
    grid_item.value = Some(value);
    stack.push(grid_item.position);
}

fn get_init_resource() -> (WaveConfig, WaveGrid, WaveStack) {
    let (config, all_possible_value) = get_config();
    let random_value = all_possible_value.iter().choose(&mut rand::thread_rng()).unwrap().clone();
    let random_x = rand::thread_rng().gen_range(0..GRID_WIDTH) as u32;
    let random_y = rand::thread_rng().gen_range(0..GRID_HEIGHT) as u32;
    let mut wave_grid = WaveGrid::new(GRID_WIDTH, GRID_HEIGHT, all_possible_value);
    let mut wave_stack = WaveStack(vec![]);
    update_grid_item_value(&mut wave_grid.0[random_y as usize][random_x as usize], random_value, &mut wave_stack);
    println!("init:{:?}\nx:{:?},y:{:?}", random_value, random_x, random_y);
    (WaveConfig(config), wave_grid, wave_stack)
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
  use super::*;
  #[test]
  fn it_works() {
    let a = GridItem::new(GridPosition::new(1, 1), HashSet::from([9,10,13,14]));
    println!("{:?}",a.get_beside_impossible_value(&get_init_resource().0.0));
    // println!("{:#?}", get_init_resource());
  }
}