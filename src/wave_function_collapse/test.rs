use std::fs;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use rand::seq::IteratorRandom;
use rand::Rng;
use serde::Deserialize;
use std::sync::RwLock;



static GRID_WIDTH: usize = 6;
static GRID_HEIGHT: usize = 6;

pub struct WaveFunctionCollapsePlugin;
impl Plugin for WaveFunctionCollapsePlugin {
    fn build(&self, app: &mut App) {
        // let (wave_config, wave_grid, wave_stack) = get_init_resource();
        // app.insert_resource(wave_config);
        // app.insert_resource(wave_grid);
        // app.insert_resource(wave_stack);
        // app.add_systems(FixedUpdate, (wave_function_collapse, update_grid_rendering).chain());
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
struct BesideImposible {
    top: HashSet<u32>,
    right: HashSet<u32>,
    bottom: HashSet<u32>,
    left: HashSet<u32>,
}

#[derive(Debug, Deserialize, Clone)]
struct ModelConfigOptions {
    // code: u32,
    beside_imposible: BesideImposible,
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
    fn get_beside_impossible_value(&self, config: &ConfigMap) -> BesideImposible {
        if let Some(_) = self.value {
            self.get_config(config).beside_imposible.clone()
        } else {
            let possible_config: Vec<BesideImposible> = self.possible_value.read().unwrap().iter().map(|possible_code| &config.get(possible_code).unwrap().beside_imposible).cloned().collect();

            let res = possible_config.iter().fold(possible_config[0].clone(), |acc, value| {
                let new_top = acc.top.intersection(&value.top).cloned().collect();
                let new_right = acc.right.intersection(&value.right).cloned().collect();
                let new_bottom = acc.bottom.intersection(&value.bottom).cloned().collect();
                let new_left = acc.left.intersection(&value.left).cloned().collect();
                BesideImposible {
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
            None
        } else {
            Some(*unrendered.iter().choose(&mut rand::thread_rng()).unwrap())
        }
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


// 获取栈内位置，得到其四个方向不可能的值，以此更新其四个方向的可能值，若更新了，将其入栈

fn wave_function_collapse(
    mut wave_grid: &mut WaveGrid,
    wave_stack: &mut WaveStack,
    wave_config: &WaveConfig,
) {
    while let Some(GridPosition {x, y}) = wave_stack.0.pop() {
        let x_usize = x as usize;
        let y_usize = y as usize;
        println!("--------------stackpop----------------- x:{},y:{}", x, y);
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~self:{:?}  self-posible{:?}", wave_grid.0[y_usize][x_usize].value, wave_grid.0[y_usize][x_usize].possible_value.read().unwrap());
        println!("?????????????????????stack:{:?}", wave_stack);
        let beside_imposible = wave_grid.0[y_usize][x_usize].get_beside_impossible_value(&wave_config.0);
        
        if y_usize != GRID_HEIGHT - 1 {
            let updated_top = update_grid(x_usize, y_usize + 1, &mut wave_grid, &beside_imposible.top);
            if updated_top {
                wave_stack.push(GridPosition::new(x, y + 1));
            }
        }

        if x_usize != GRID_WIDTH - 1 {
            let updated_right = update_grid(x_usize + 1, y_usize, &mut wave_grid, &beside_imposible.right);
            if updated_right {
                wave_stack.push(GridPosition::new(x + 1, y));
            }
        }
        
        if y_usize != 0 {
            let updated_bottom = update_grid(x_usize, y_usize - 1, &mut wave_grid, &beside_imposible.bottom);
            if updated_bottom {
                wave_stack.push(GridPosition::new(x, y - 1));
            }
        }
        
        if x_usize != 0 {
            let updated_left = update_grid(x_usize - 1, y_usize, &mut wave_grid, &beside_imposible.left);
            if updated_left {
                wave_stack.push(GridPosition::new(x - 1, y));
            }
        }

    }
    let new_position = wave_grid.get_rand_unrendered();
    if let Some(new_position) = new_position {
        println!("grid:{:?}", wave_grid.0);
        let wave_grid_item = &mut wave_grid.0[new_position.y as usize][new_position.x as usize];
        if let Some(new_value) = wave_grid_item.possible_value.read().unwrap().iter().choose(&mut rand::thread_rng()) {
            wave_grid_item.value = Some(*new_value);
            wave_stack.push(new_position);
            println!("new_iter:{:?}\n{:?}\n", new_value, new_position);
        };
    }
}

// 有更新返回true， 否则返回false
fn update_grid(
    x: usize,
    y: usize,
    wave_grid: &mut WaveGrid,
    beside_imposible: &HashSet<u32>,
) -> bool {
    let grid = &mut wave_grid.0[y][x];

    if grid.value.is_some() {
        return false;
    }
    let mut removed = false;
    println!("position:x:{:?},y:{:?}", x, y);
    println!("grid-possible:{:?}", grid.possible_value.read().unwrap());
    println!("grid-impossible:{:?}", beside_imposible);
    beside_imposible.iter().for_each(|code| {
        let removed_flag = grid.possible_value.write().unwrap().remove(code);
        println!("removed-code:{:?}---last:{:?}", code, grid.possible_value.read().unwrap());
        removed = removed || removed_flag;
    });
    if grid.possible_value.read().unwrap().is_empty() {
        panic!("errrrrrrrrrr");
    }
    if grid.possible_value.read().unwrap().len() == 1 {
        grid.value = grid.possible_value.read().unwrap().iter().next().cloned();
    }
    removed
}

fn update_grid_rendering(
    config: &WaveConfig,
    wave_grid: &mut WaveGrid,

) {
    for grid_row in wave_grid.0.iter_mut() {
        for grid_item in grid_row.iter_mut() {
            if grid_item.value.is_none() || grid_item.rendered {
                continue;
            }
            let (position, asset_path_str) = grid_item.get_model_bundle(&config.0);
            
            if let Some(_) = grid_item.value {
                
                grid_item.rendered = true;
            }
        }
    }
}

fn get_init_resource() -> (WaveConfig, WaveGrid, WaveStack) {
    let (config, all_possible_value) = get_config();
    let random_value = all_possible_value.iter().choose(&mut rand::thread_rng()).unwrap().clone();
    let random_x = rand::thread_rng().gen_range(0..GRID_WIDTH) as u32;
    let random_y = rand::thread_rng().gen_range(0..GRID_HEIGHT) as u32;
    let mut wave_grid = WaveGrid::new(GRID_WIDTH, GRID_HEIGHT, all_possible_value);
    // let wave_stack = WaveStack(vec![GridPosition::new(random_x, random_y)]);
    // wave_grid.0[random_y as usize][random_x as usize].value = Some(random_value);
    let wave_stack = WaveStack(vec![GridPosition::new(4, 2)]);
    wave_grid.0[2 as usize][4 as usize].value = Some(7);
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
    // let a = GridItem::new(GridPosition::new(1, 1), HashSet::from([9,10,13,14]));
    // println!("{:?}",a.get_beside_impossible_value(&get_init_resource().0.0));
    // println!("{:#?}", get_init_resource());
    let (wave_config, mut wave_grid, mut wave_stack) = get_init_resource();
    while !wave_stack.0.is_empty() {
        wave_function_collapse(&mut wave_grid, &mut wave_stack, &wave_config);
        update_grid_rendering(&wave_config, &mut wave_grid);
    }
    
  }
}