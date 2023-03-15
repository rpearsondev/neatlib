use std::{marker::PhantomData, time::Duration};
use wasm_stopwatch::Stopwatch;
use crate::{neat::{trainer::{fitness::{fitness_setter::{FitnessSetter}}, neat_trainer::NeatTrainer}, genome::neat::NeatGenome, population::GenerationMember}, distributed_compute::{queues::{CPU_DISTRIBUTED_WORK_QUEUE, CPU_DISTRIBUTED_RESULTS_QUEUE}, models::to_agent_work_item::ToAgentWorkItem}};

/*
The responsibilities of the activation strategies:
- Activate the phenomes
- Set the fitness on the members
- Update the run context with novelty component quantized values.
*/

pub struct CpuDistibuted<'a>{
    neat_trainer: &'a mut NeatTrainer,
    use_new: PhantomData<u8>
}
impl<'a> CpuDistibuted<'a>{
    pub fn new(neat_trainer: &'a mut NeatTrainer) -> Self{
        Self{
            neat_trainer,
            use_new: PhantomData
        }
    }
    pub fn compute(&mut self, fitness_setter: &mut FitnessSetter) {
        let members = &mut self.neat_trainer.members;
 
        let mut work_to_agent_items: Vec<ToAgentWorkItem> = Vec::with_capacity(members.len());
        for m in members.iter(){
            work_to_agent_items.push(ToAgentWorkItem { member: m.clone() });
        }

        //set the latest component novelty store
        {
            let mut queue = CPU_DISTRIBUTED_WORK_QUEUE.lock().unwrap();
            queue.current_generation = self.neat_trainer.run_context.current_generation;
            queue.latest_novelty_component_store = Some(self.neat_trainer.run_context.novelty_component_store.clone());
            queue.latest_configuration = Some(self.neat_trainer.configuration.clone());
            queue.to_agent_queue = work_to_agent_items.clone();
        }

        //wait for the queue to empty (get pulled by the agents)
        loop{
            let queue_size;
            {
                let queue = CPU_DISTRIBUTED_WORK_QUEUE.lock().unwrap();
                queue_size = queue.to_agent_queue.len();
            }
            
            if queue_size == 0{
                break;
            }else{
                std::thread::sleep(Duration::from_millis(5));
            }
        }

         //wait result queue to fill
         let sw = Stopwatch::new();
         loop{
            let existing_member_ids;
            {
                let results_queue = CPU_DISTRIBUTED_RESULTS_QUEUE.lock().unwrap();
                existing_member_ids = results_queue.from_agent_results_queue.iter().map(|x| x.genome_id.clone()).collect::<Vec<uuid::Uuid>>();
            }

            let missing_members_count = members.iter().filter(|x| !existing_member_ids.contains(&x.genome.id) ).count();
            
            if missing_members_count == 0{
                break;
            }else{
                println!("Waiting for results");
                std::thread::sleep(Duration::from_millis(5));

                if sw.get_time() > 0.5{
                    println!("Waiting too long for results, re-queueing missing members");
                    {
                        
                        let results_queue = CPU_DISTRIBUTED_RESULTS_QUEUE.lock().unwrap();
                        let existing_member_ids = results_queue.from_agent_results_queue.iter().map(|x| x.genome_id.clone()).collect::<Vec<uuid::Uuid>>();
                        let missing_members = members.iter().filter(|x| !existing_member_ids.contains(&x.genome.id) ).map(|m| m.clone()).collect::<Vec<GenerationMember<NeatGenome>>>();
                        
                        {
                            let mut work_queue = CPU_DISTRIBUTED_WORK_QUEUE.lock().unwrap();
                            for m in &missing_members{
                                work_queue.to_agent_queue.push(ToAgentWorkItem { member: m.clone()});
                            }
                        }

                        println!("Re-queued {} members", missing_members.len());
                    }
                    std::thread::sleep(Duration::from_millis(300));
                }
            }
        }

        let results_queue;
        {
            let mut queue = CPU_DISTRIBUTED_RESULTS_QUEUE.lock().unwrap();
            results_queue = queue.from_agent_results_queue.clone();
            queue.from_agent_results_queue.clear();
        }
       
        for result in results_queue{
            fitness_setter.set_fitness(result.genome_id, result.fitness.clone())
        }

    }
    pub fn new_generation(&mut self){
        self.neat_trainer.new_generation()
    }
}