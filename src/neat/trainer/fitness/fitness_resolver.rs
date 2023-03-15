use crate::{common::{NeatFloat}};
use super::{Fitness, novelty_component_store::{NoveltyComponentStore}, number_line::{ComponentNoveltyQuantizedValue}};

pub struct FitnessResolver<'a>{
    novelty_component_store: &'a NoveltyComponentStore,
    objective_fitness: NeatFloat,
    total_novelty: NeatFloat,
    novelty_component_count: u64,
    outcome_novelty_quantized_values: Vec<ComponentNoveltyQuantizedValue>
}

impl<'a> FitnessResolver<'a>{
    pub fn new(novelty_component_store: &'a NoveltyComponentStore) -> Self{
        Self{
            novelty_component_store,
            objective_fitness: 0.0,
            total_novelty: 0.0,
            novelty_component_count: 0,
            outcome_novelty_quantized_values: Vec::new()
        }
    }
    pub fn add_objective_fitness_component(&mut self, component_id:u32, importance: NeatFloat, expected_value: NeatFloat, actual_value: NeatFloat){
        if expected_value.is_subnormal(){
            println!("expected_value is subnormal for component_id {}", component_id);
            return;
        }
        if actual_value.is_subnormal(){
            println!("actual_value is subnormal for component_id {}", component_id);
            return;
        }
        self.objective_fitness += importance - (importance * NeatFloat::powi(NeatFloat::abs(expected_value - actual_value), 2));
    }
    pub fn add_objective_fitness_component_with_novelty(&mut self, component_id: u32, importance: NeatFloat, expected_value: NeatFloat, actual_value: NeatFloat, quantization_to_int_multiplier: i64){
        self.add_objective_fitness_component(component_id, importance, expected_value, actual_value);
        self.add_novelty_component(component_id, actual_value, quantization_to_int_multiplier);
    }
    pub fn add_reward(&mut self, _component_id: u32, value: NeatFloat){
        self.objective_fitness += value;
    }
    pub fn add_punishment(&mut self, _component_id: u32, value: NeatFloat){
        self.objective_fitness -= value;
    }
    
    pub fn add_novelty_component(&mut self, component_id: u32, actual_value: NeatFloat, quantization_to_int_multiplier: i64){
        let quantized_value = (actual_value * quantization_to_int_multiplier as f32) as i64;
        let novelty = self.novelty_component_store.get_novelty_component_score(component_id, quantized_value) / 100.0;
        self.outcome_novelty_quantized_values.push(ComponentNoveltyQuantizedValue {component_id, quantized_value: quantized_value, count: 1 });
        self.total_novelty += novelty;
        self.novelty_component_count += 1;
    }
    pub fn compute(&mut self) -> Fitness{
        let mut outcome_novelty = 0.0;
        if self.novelty_component_count > 0{
            outcome_novelty = self.total_novelty / self.novelty_component_count as f32;
        }

        Fitness{
            objective_fitness: self.objective_fitness,
            outcome_novelty: outcome_novelty,
            outcome_novelty_quantized_values: Some(self.outcome_novelty_quantized_values.clone())
        }
    }
}