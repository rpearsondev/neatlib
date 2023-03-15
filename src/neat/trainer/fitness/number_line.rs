

use std::{marker::PhantomData, collections::BTreeMap};

use serde::{Deserialize, Serialize};

use crate::common::NeatFloat;
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct NumberLine{
    pub numbers: BTreeMap<i64, u64>
}
//This struct needs some attention.
impl NumberLine{
    pub fn new() -> Self{
        Self {numbers: BTreeMap::new() }
    }

    pub fn get_novelty_distance(&self, new_number: i64) -> DistanceResult{  
        let existing_opt= self.numbers.get(&new_number);
        if existing_opt.is_some(){
            let existing = existing_opt.unwrap();
            return DistanceResult::with_distance(1.0 / u64::max(*existing, 2) as NeatFloat);
        }else{
            
            let mut less_than_range = self.numbers.range(i64::MIN..new_number);
            let mut more_than_range = self.numbers.range(new_number..i64::MAX);

            let first_less_than_opt = less_than_range.next_back();
            
            let mut distance = i64::MAX;
            if first_less_than_opt.is_some(){
                let (first_less_than, _) = first_less_than_opt.unwrap();
                distance = i64::abs(new_number - first_less_than);
            }

            let first_more_than_opt = more_than_range.next();
            if first_more_than_opt.is_some(){
                let (first_more_than, _) = first_more_than_opt.unwrap();
                let temp_distance = i64::abs(first_more_than - new_number);
                if temp_distance < distance{
                    distance = temp_distance;
                }
            }
            DistanceResult::with_distance(distance as NeatFloat)
        }
        
       
    }

    pub fn add_quantized_values(&mut self, values: &Vec<ComponentNoveltyQuantizedValue>) {  
        for value in values{
            let existing_value_opt = self.numbers.get_mut(&value.quantized_value);
            if existing_value_opt.is_some(){
                let existing_value = existing_value_opt.unwrap();
                *existing_value += value.count;
            }else{
                self.numbers.insert(value.quantized_value, value.count);
            }
          
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct DistanceResult{
    pub distance_to_nearest_neighbor: NeatFloat,
    use_method: PhantomData<u8>
}
impl DistanceResult{
    pub fn zero() -> Self{
        Self{
            distance_to_nearest_neighbor: 0.0,
            use_method: PhantomData
        }
    }
    pub fn default() -> Self{
        Self{
            distance_to_nearest_neighbor: 1.0,
            use_method: PhantomData
        }
    }
    pub fn with_distance(distance: NeatFloat) -> Self{
        Self{
            distance_to_nearest_neighbor: distance,
            use_method: PhantomData
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct ComponentNoveltyQuantizedValue{
    pub component_id: u32,
    pub quantized_value: i64,
    pub count: u64
}

#[cfg(test)]
mod number_line_test{
    use crate::{common::random::Random, neat::trainer::fitness::number_line::ComponentNoveltyQuantizedValue};

    use super::NumberLine;

    #[test]
    #[ignore = "For development only"]
    fn print_check(){
        let mut number_line = NumberLine::new();
        for _i in 0..10_000{
            let new_number = Random::gen_range_i64(-1000, 1000);
            number_line.add_quantized_values(&vec![ComponentNoveltyQuantizedValue{component_id: 0, quantized_value: new_number, count: 1}]);
        }

        for _ in 0..100{
            let new_number = Random::gen_range_i64(-1000, 1000);
            println!("{}",number_line.get_novelty_distance(new_number as i64).distance_to_nearest_neighbor);
        }
    }

    #[test]
    #[ignore = "For development only"]
    fn print_check_2(){
        let mut number_line = NumberLine::new();
    
        number_line.add_quantized_values(&vec![
            ComponentNoveltyQuantizedValue{ component_id: 0, quantized_value: 8, count: 1},
            ComponentNoveltyQuantizedValue{ component_id: 0, quantized_value: 12, count: 1},
            ]);
     
     
        println!("{}",number_line.get_novelty_distance(1).distance_to_nearest_neighbor);
        println!("{}",number_line.get_novelty_distance(2).distance_to_nearest_neighbor);
        println!("{}",number_line.get_novelty_distance(3).distance_to_nearest_neighbor);
        println!("{}",number_line.get_novelty_distance(4).distance_to_nearest_neighbor);
        println!("{}",number_line.get_novelty_distance(5).distance_to_nearest_neighbor);
        println!("{}",number_line.get_novelty_distance(6).distance_to_nearest_neighbor);
        println!("{}",number_line.get_novelty_distance(7).distance_to_nearest_neighbor);
        println!("{}",number_line.get_novelty_distance(8).distance_to_nearest_neighbor);
        println!("{}",number_line.get_novelty_distance(9).distance_to_nearest_neighbor);
        println!("{}",number_line.get_novelty_distance(10).distance_to_nearest_neighbor);
        println!("{}",number_line.get_novelty_distance(11).distance_to_nearest_neighbor);
        println!("{}",number_line.get_novelty_distance(12).distance_to_nearest_neighbor);
        
    }

    #[test]
    #[ignore = "For development only"]
    //cargo test --release neat::trainer::fitness::number_line::number_line_test::million_performance -- --nocapture 
    fn million_performance(){
        let mut number_line = NumberLine::new();

        for _i in 0..10_000{
            let new_number = Random::gen_range_i64(-1000, 1000);
            number_line.add_quantized_values(&vec![ComponentNoveltyQuantizedValue{ component_id: 0, quantized_value: new_number, count: 1}]);
        }

        for _ in 0..1000_000{
            let new_number = Random::gen_range_i64(-10, 10);
            number_line.get_novelty_distance(new_number);
        }
        println!("{:?}", number_line);
    }
}