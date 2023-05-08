use uuid::Uuid;

use crate::common::NeatFloat;

pub trait Phenome{
    fn activate(&self, sensor_values: &Vec<NeatFloat>) -> Vec<NeatFloat>;
    fn get_id(&self) -> Uuid;
}