use std::sync::RwLock;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use rand::prelude::IteratorRandom;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
}
impl GridPosition {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct BesideImpossible {
    pub top: HashSet<u32>,
    pub right: HashSet<u32>,
    pub bottom: HashSet<u32>,
    pub left: HashSet<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ModelConfigOptions {
    beside_impossible: BesideImpossible,
    asset_model: String
}

pub type ConfigMap = HashMap<u32, ModelConfigOptions>;

#[derive(Resource, Debug)]
pub struct WaveConfig(pub ConfigMap);

#[derive(Debug)]
pub struct GridItem {
    value: Option<u32>,
    pub possible_value: RwLock<HashSet<u32>>,
    pub position: GridPosition,
    pub rendered: bool,
}

impl GridItem {
    pub fn new(position: GridPosition, possible_value: HashSet<u32>) -> Self {
        let possible_value = RwLock::new(possible_value);
        Self {
            value: None,
            possible_value,
            position,
            rendered: false
        }
    }
    pub fn get_value(&self) -> Option<u32> {
        self.value
    }
    pub fn get_model_bundle(&self, config: &ConfigMap) -> (&GridPosition, String) {
        (
            &self.position,
            self.get_config(config).asset_model.clone()
        )
    }
    // 必须value不为空才可调用
    pub fn get_config<'a>(&self, config: &'a ConfigMap) -> &'a ModelConfigOptions {
        config.get(&self.value.unwrap()).unwrap()
    }
    pub fn get_beside_impossible_value(&self, config: &ConfigMap) -> BesideImpossible {
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

#[derive(Debug)]
pub struct WaveStack(Vec<GridPosition>);
impl WaveStack {
    fn new() -> Self {
        Self(Vec::new())
    }
    pub fn push(&mut self, position: GridPosition) {
        if self.0.contains(&position) {
            return;
        }
        self.0.push(position);
    }
    pub fn pop(&mut self) -> Option<GridPosition> {
        self.0.pop()
    }
}

#[derive(Debug)]
pub struct GridVec(Vec<Vec<GridItem>>);
impl GridVec {
    pub fn get_mut_value(&mut self) -> &mut Vec<Vec<GridItem>> {
        &mut self.0
    }
    pub fn get_item(&self, x: u32, y: u32) -> &GridItem {
        & self.0[y as usize][x as usize]
    }
    pub fn get_mut_item(&mut self, x: u32, y: u32) -> &mut GridItem {
        &mut self.0[y as usize][x as usize]
    }
}

#[derive(Resource, Debug)]
pub struct WaveGrid {
    pub grid_value: GridVec,
    pub stack: WaveStack,
    not_collapsed_set: RwLock<HashSet<GridPosition>>
}

impl WaveGrid {
    pub fn new(width: usize, height: usize, all_possible_value: HashSet<u32>) -> Self {
        let mut vector: Vec<Vec<GridItem>> = vec![];
        let not_collapsed_set = RwLock::new(HashSet::new());
        for row_index in 0..height {
            let mut row_vec = vec![];
            for col_index in 0..width {
                let x = col_index as u32;
                let y = row_index as u32;
                row_vec.push(GridItem::new(
                    GridPosition::new(x, y),
                    all_possible_value.clone()
                ));
                not_collapsed_set.write().unwrap().insert(GridPosition::new(x, y));
            }
            vector.push(row_vec);
        }
        Self {
            grid_value: GridVec(vector),
            stack: WaveStack::new(),
            not_collapsed_set,
        }
    }

    pub fn update_grid(
        &mut self,
        position: GridPosition,
        beside_impossible: &HashSet<u32>,
    ) {
        let grid_item = self.grid_value.get_mut_item(position.x, position.y);
        if grid_item.get_value().is_some() {
            return;
        }
        let old_len = grid_item.possible_value.read().unwrap().len();
        grid_item.possible_value.write().unwrap().retain(|value| !beside_impossible.contains(value));
        if old_len != grid_item.possible_value.read().unwrap().len() {
            self.stack.push(grid_item.position.clone());
        }
        if grid_item.possible_value.read().unwrap().len() == 1 {
            let value = grid_item.possible_value.read().unwrap().iter().next().cloned().unwrap();
            self.update_grid_item_value(position, value);
        }
    }
    
    pub fn update_grid_item_value(&mut self, position: GridPosition, value: u32) {
        self.not_collapsed_set.write().unwrap().remove(&position);
        self.grid_value.get_mut_item(position.x, position.y).value = Some(value);
        self.stack.push(position);
    }

    pub fn get_rand_not_collapse(&self) -> Option<GridPosition> {
        self.not_collapsed_set.read().unwrap().iter().cloned().choose(&mut rand::thread_rng())
    }
}

