use crate::neat::trainer::neat_trainer::NeatTrainer;

use super::{ cpu_parallel::CpuParallel, cpu_distributed::CpuDistibuted};

#[derive(Default)]
pub struct ActivationStrategies;

impl ActivationStrategies{
    pub fn get_cpu_parallel(neat_trainer: &mut NeatTrainer) -> CpuParallel{
        CpuParallel::new(neat_trainer)
    }
    pub fn get_cpu_distibuted(neat_trainer: &mut NeatTrainer) -> CpuDistibuted{
        CpuDistibuted::new(neat_trainer)
    }
}