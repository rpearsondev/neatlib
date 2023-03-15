use std::thread;

use crate::{neat::trainer::fitness::{fitness_resolver::FitnessResolver}, phenome::Phenome};

use super::distributed_agent::DistributedAgent;

pub struct DistributedRun<F> where F:Fn(&Phenome, &mut FitnessResolver) + std::marker::Sync{
    pub agent: DistributedAgent,
    setup: fn(),
    set_individual_fitness: F
}

impl<F> DistributedRun<F> where F:Fn(&Phenome, &mut FitnessResolver) + std::marker::Sync{
    pub fn new(
        agent: DistributedAgent,
        set_individual_fitness: F,
        host_setup: fn()
    ) -> Self
        where F:Fn(&Phenome, &mut FitnessResolver) + std::marker::Sync {
        Self { 
            agent,
            setup: host_setup,
            set_individual_fitness: set_individual_fitness
     }
    }
    pub fn run(&self) {
        if self.agent.is_host{
            let agent = self.agent.clone();
            let runner_join_handle = thread::spawn(move ||{
                agent.start_service_host();
            });
            (self.setup)();

            runner_join_handle.join().unwrap();
        }else {
            self.agent.start_client_poller(&self.set_individual_fitness);
        }
        
    }
}