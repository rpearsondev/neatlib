use std::marker::PhantomData;
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};
use crate::{neat::{trainer::{fitness::{fitness_setter::{FitnessSetter}, Fitness, fitness_resolver::FitnessResolver, novelty_component_store::NoveltyComponentStore}, neat_trainer::NeatTrainer, configuration::Configuration}, population::GenerationMember, genome::neat::NeatGenome}, phenome::Phenome, common::NeatFloat, cpu_phenome::CpuPhenome};

/*
The responsibilities of the activation strategies:
- Activate the phenomes
- Set the fitness on the members
- Update the run context with novelty component quantized values.

*/

pub struct CpuParallel<'a>{
    neat_trainer: &'a mut NeatTrainer,
    use_new: PhantomData<u8>
}
impl<'a> CpuParallel<'a>{
    pub fn new(neat_trainer: &'a mut NeatTrainer) -> Self{
        Self{
            neat_trainer,
            use_new: PhantomData
        }
    }
    pub fn compute<F>(&mut self, set_individual_fitness: F, fitness_setter: &mut FitnessSetter) where F:Fn(&dyn Phenome, &mut FitnessResolver) + std::marker::Sync{
        let members = &mut self.neat_trainer.members;
        let novelty_component_store = &self.neat_trainer.run_context.novelty_component_store;

        let results = compute_fitnesses_cpu(set_individual_fitness, &self.neat_trainer.configuration, members, novelty_component_store);
    
        for (id, fitness) in results{
            fitness_setter.set_fitness(id, fitness)
        }
    }
    pub fn new_generation(&mut self){
        self.neat_trainer.new_generation()
    }
}

pub fn compute_fitnesses_cpu<F>(set_individual_fitness: F, _configuration: &Configuration, members: &mut Vec<GenerationMember<NeatGenome>>, novelty_component_store: &NoveltyComponentStore ) -> Vec<(uuid::Uuid, Fitness)> where F:Fn(&dyn Phenome, &mut FitnessResolver) + std::marker::Sync {
    let fitnesses = members.par_iter_mut().map(|m| {
        let mut fitness_resolver = FitnessResolver::new(&novelty_component_store);
        let phenome = CpuPhenome::from_network_schema(&m.genome);
        
        //add some novelty for structure
        fitness_resolver.add_novelty_component(1001, m.genome.genes.get_complexity(), 1);
        fitness_resolver.add_novelty_component(1002, m.genome.genes.nodes.iter().count() as NeatFloat, 1);
        fitness_resolver.add_novelty_component(1003, phenome.layers.len() as NeatFloat, 1);

        let _ = &set_individual_fitness(&phenome, &mut fitness_resolver);
        (m.genome.id , fitness_resolver.compute())
    }).collect::<Vec<(uuid::Uuid, Fitness)>>();
    fitnesses
}