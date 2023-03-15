use crate::{neat::{genome::genome::Genome, trainer::neat_trainer::NeatTrainer}};
use super::Fitness;

struct MemberFitness{
    pub id: uuid::Uuid,
    pub fitness: Fitness
}

//The purpose of this struct is to centralize the logic of fitness setting.
pub struct FitnessSetter{
    fitnesses: Vec<MemberFitness>
}

impl FitnessSetter{
    pub fn new() -> Self{
        Self{
            fitnesses: Vec::new()
        }
    }
    pub fn set_fitness(&mut self,id: uuid::Uuid, fitness:Fitness){
        self.fitnesses.push(MemberFitness{id, fitness});
    }
    pub fn commit(&self, trainer: &mut NeatTrainer){
        for member_fitness in &self.fitnesses{

            if member_fitness.fitness.outcome_novelty_quantized_values.is_some(){
                trainer.run_context.novelty_component_store.add_quantized_values(&member_fitness.fitness.outcome_novelty_quantized_values.as_ref().unwrap());
            }

            let member = trainer.members_map.get(&member_fitness.id.as_u64_pair().0);
            if member.is_some(){
                if member_fitness.fitness.objective_fitness.is_subnormal(){
                    panic!("fitness is subnormal for member {}", member_fitness.id);
                }
                let member = &mut trainer.members[*member.unwrap()];
                member.genome.set_objective_fitness(member_fitness.fitness.objective_fitness);
                member.genome.set_novelty(member_fitness.fitness.outcome_novelty);
            }
        }
    }
}