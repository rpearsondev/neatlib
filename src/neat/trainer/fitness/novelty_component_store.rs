use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;
use serde::{Deserialize, Serialize};

use crate::common::NeatFloat;
use super::number_line::{NumberLine, DistanceResult, ComponentNoveltyQuantizedValue};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NoveltyComponentStore{
    pub components: HashMap<u32, NumberLine, nohash_hasher::BuildNoHashHasher<u64>>
}
impl NoveltyComponentStore{
    pub fn new() -> Self{
        Self{
            components: HashMap::with_hasher(BuildNoHashHasher::default()),
        }
    }
    pub fn clear(&mut self){
        self.components.clear();
    }
    pub fn get_novelty_component_score(&self, component_id: u32, quantized_value: i64) -> NeatFloat{
        let line_opt = self.components.get(&component_id);
        if line_opt.is_some(){
            let line = line_opt.unwrap();
            let distance_result = line.get_novelty_distance(quantized_value);
            return distance_result.distance_to_nearest_neighbor as NeatFloat
        }
        DistanceResult::default().distance_to_nearest_neighbor
    }
    pub fn add_quantized_values(&mut self, quantized_values: &Vec<ComponentNoveltyQuantizedValue>) {
        for quantized_value in quantized_values{
            let line_opt = self.components.get_mut(&quantized_value.component_id);
            if line_opt.is_some(){
                let line = line_opt.unwrap();
                line.add_quantized_values(&vec![*quantized_value]);
            }else{
                let mut new_line = NumberLine::new();
                new_line.add_quantized_values(&vec![*quantized_value]);
                self.components.insert(quantized_value.component_id, new_line);
            }
        }
    }
}

