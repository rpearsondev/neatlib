use serde::{Deserialize, Serialize};
use crate::common::NeatFloat;
use super::species_member::SpeciesMember;


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SpeciesMetric{
    pub max: NeatFloat,
    pub max_positively_adjusted: NeatFloat,
    pub min: NeatFloat,
    pub average: NeatFloat,
    pub positively_adjusted_average: NeatFloat,
    lowest_comparison_point: NeatFloat,
    last_generation_max: NeatFloat,
    pub last_generation_improved: u32
}
impl SpeciesMetric{
    pub fn new() -> Self{
        Self{
            average: 0.0,
            positively_adjusted_average: 0.0,
            min: 0.0,
            max: 0.0,
            max_positively_adjusted: 0.0,
            lowest_comparison_point: 0.0,
            last_generation_improved: 0,
            last_generation_max: 0.0,
        }
    }
    pub fn new_generation<F>(&mut self, lowest_comparison_point: NeatFloat, members: &Vec<SpeciesMember>, metric_selector: F, current_generation: u32) where F: Fn(&SpeciesMember) -> NeatFloat{

        self.last_generation_max = self.max;

        if members.len() == 0 {
            self.average = 0.0;
            self.positively_adjusted_average = 0.0;
            self.min = 0.0;
            self.max = 0.0;
            self.lowest_comparison_point = 0.0;
        }

        let mut sum = 0.0;
        for member in members{
            let value = metric_selector(member);
            if value > self.max{
                self.max = value;
            }
            if value < self.min{
                self.min = value;
            }
            sum += value;
        }

        self.lowest_comparison_point = lowest_comparison_point;
        self.average = sum / members.len() as NeatFloat;
        self.positively_adjusted_average = f32::abs(lowest_comparison_point - self.average);
        
        if self.max > self.last_generation_max{
            self.last_generation_improved = current_generation;
        }

        self.max_positively_adjusted =f32::abs(lowest_comparison_point - self.max);
    }
}