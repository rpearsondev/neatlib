use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc::Sender;
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;
use crate::common::NeatFloat;
use crate::neat::genome::neat::mutation_mode::MutationMode;
use crate::neat::population::speciation::speciation::Speciation;
use crate::neat::population::speciation::species::Species;
use crate::neat::trainer::config_regulators::config_regulator_handler::ConfigRegulatorHandler;
use crate::neat::trainer::run_signals::run_signals::RunSignals;
use crate::neat::genome::neat::NeatGenome;
use crate::neat::genome::genome::Genome;
use crate::neat::population::GenerationMember;
use serde::{Deserialize, Serialize};
use super::configuration::{Configuration};
use super::configuration_defaults::ConfigurationDefaults;
use super::neat_trainer_host::from_host_events::FromHostEvent;
use super::neat_trainer_host::models::run_stats::RunStats;
use super::config_regulators::config_regulator::ConfigRegulator;
use super::run_context::{RunContext};

#[derive(Serialize, Deserialize)]
pub struct NeatTrainer{
    pub members: Vec<GenerationMember<NeatGenome>>,
    pub members_map: HashMap<u64, usize, nohash_hasher::BuildNoHashHasher<u64>>,
    pub configuration: Configuration,
    pub run_context: RunContext,
    has_printed_summary: bool,
    #[serde(skip_serializing,skip_deserializing)]
    pub event_sender: Option<Sender<FromHostEvent>>,
    pub config_regulators: Vec<ConfigRegulator>
}
impl NeatTrainer{
    pub fn new(configuration: Configuration) -> Self {
        let run_context = RunContext::new(configuration.node_genes.len()+1, configuration.speciation_use_best_seed_bank.unwrap_or_default());
        let regulators = ConfigurationDefaults::get_default_regulators(&configuration);
        NeatTrainer {
            configuration,
            run_context,
            members: Vec::new(),
            members_map: HashMap::with_capacity_and_hasher(0, BuildNoHashHasher::default()),
            has_printed_summary: false,
            event_sender: None,
            config_regulators: regulators
        }
    }
    pub fn new_generation(&mut self) {
        {
            optick::next_frame();

            if self.has_met_success(){
                return;
            }

            self.set_best_member_and_worst_fitness();

            if self.configuration.print_summary_interval.is_some() && self.run_context.current_generation % self.configuration.print_summary_interval.unwrap() == 0 {
                self.print_summary(self.configuration.print_summary_number_of_species_to_show);
            }

            Speciation::speciate(&mut self.members, &self.configuration, &mut self.run_context);

            self.recompute_indexes();

            let run_context = &mut self.run_context;
            for m in self.members.iter_mut(){
                
                let mut mutation_mode = MutationMode::Steady;

                if m.number_of_generations_since_species_improved > 4 {
                    mutation_mode = MutationMode::Optimistic;
                }else if m.number_of_generations_since_species_improved > 8{
                    mutation_mode = MutationMode::FoolHardy;
                }else if m.number_of_generations_since_species_improved > 10{
                    mutation_mode = MutationMode::ShootForTheMoon;
                }
                
                m.genome.mutate(&mut self.configuration, run_context, mutation_mode);
            }
            
            let run_signals = RunSignals::new(&run_context);

            for config_regulator in &self.config_regulators{
                if run_context.current_generation > config_regulator.start_generation{
                    ConfigRegulatorHandler::handle(config_regulator, &mut self.configuration, &run_signals)
                }
            }

            run_context.increment_generation();
        }
        self.send_event(FromHostEvent::ConfigUpdate(self.configuration.clone()));
        self.send_event(FromHostEvent::GenerationChange(self.run_context.current_generation));
        self.send_event(FromHostEvent::RegulatorUpdate(self.config_regulators.clone()));
    }
    pub fn reset(&mut self){
        self.members.clear();
        self.run_context = RunContext::new(self.configuration.node_genes.len()+1, self.configuration.speciation_use_best_seed_bank.unwrap_or_default());
        self.recompute_indexes();

        self.send_event(FromHostEvent::ConfigUpdate(self.configuration.clone()));
        self.send_event(FromHostEvent::RegulatorUpdate(self.config_regulators.clone()));
        self.send_event(FromHostEvent::RunStats(RunStats::new(self)));
    }
    pub fn are_all_fitnesses_set(&self) -> bool{
        
        self.members.iter().all(|m| m.genome.objective_fitness.is_some() && m.genome.objective_fitness.unwrap() >= 0.0)
    }
    pub fn print_summary(&mut self, number_of_species_to_show: usize){
        
        let num_species = self.run_context.species_index.len();
        let mut best_in_latest_generation : Option<NeatFloat> = None;
        for m in &self.members{
            if m.genome.objective_fitness.is_some() {
                if best_in_latest_generation.is_none() || m.genome.objective_fitness.unwrap() > best_in_latest_generation.unwrap(){
                    best_in_latest_generation = m.genome.objective_fitness;
                }
            }
        }
        println!("summary --- - -- - -");

        let mut complexity_score = 0.0;
        if self.run_context.best_member_so_far.is_some(){
            complexity_score = self.run_context.best_member_so_far.as_ref().unwrap().genome.get_complexity();
        }
        println!("gen:{}\tpop: {}\tnum_species: {}\t best_in_gen:{:.6}\t thres:{:.4} \tcomplexity:{:.4}",
            self.run_context.current_generation,
            self.members.len(),
            num_species,
            best_in_latest_generation.unwrap_or_default(),
            self.configuration.speciation_genetic_distance_threshold,
            complexity_score
        );
    
        if number_of_species_to_show > 0 && self.run_context.species_index.len() > 0{
            let mut species = self.run_context.species_index.iter().map(|s| s.1.clone()).collect::<Vec<Species>>();
            species.sort();
            let mut best_species = self.run_context.species_index.values().find(|_a| true).unwrap();
            for (_n, species) in &self.run_context.species_index{
                if species.objective_fitness.average > best_species.objective_fitness.average {
                    best_species = &species;
                }
            }

            for species in species.iter_mut().take(number_of_species_to_show){
                let distance_from_best = species.primary.genome.get_genetic_difference_distance_from(&best_species.primary.genome, NeatFloat::MAX); 
                println!("id: {}\tmem no: {}\tav.fit: {:.4}\t ofs'g:{}\tdis f'm b'st:{:.2}\tadj-fit:{:.4}",
                species.id.simple(),
                species.members.len(),
                species.objective_fitness.average,
                species.allowed_number_of_offspring_based_on_objective_fitness,
                distance_from_best,
                species.adjusted_average_objective_fitness_based_on_member_range
            );
        }
    }
    }
    pub fn has_met_success(&mut self) -> bool{
        
        if self.run_context.best_member_so_far.is_none() {
            return false;
        }
        let best = self.run_context.best_member_so_far.as_ref().unwrap();
        if best.genome.objective_fitness.is_none(){
            return false;
        }
        if best.genome.objective_fitness.unwrap_or_default() >= self.configuration.success_threshold{
            if self.configuration.print_summary_interval.is_some() && !self.has_printed_summary{
                self.print_summary(999);
                self.has_printed_summary = true;
            }
            return true;
        }
        return false;
    }
    pub fn get_best_member_so_far(&mut self) -> &Option<GenerationMember<NeatGenome>> {
        &self.run_context.best_member_so_far
    }
    pub fn get_best_member_in_last_gen(&mut self) -> &Option<GenerationMember<NeatGenome>> {
        &self.run_context.get_best_member_in_this_gen
    }
    pub fn get_current_generation(&self) -> u32{
        self.run_context.current_generation
    }
    fn set_best_member_and_worst_fitness(&mut self){
        self.run_context.get_best_member_in_this_gen = None;
        for m in &self.members{
            if self.run_context.best_member_so_far.is_none() {
                self.run_context.best_member_so_far = Some(m.clone());
                self.send_event(FromHostEvent::BestNewGenome(m.clone()))
            } 

            if self.run_context.get_best_member_in_this_gen.is_none() {
                self.run_context.get_best_member_in_this_gen = Some(m.clone());
            } 
    
            let best_member_ever_fitness= self.run_context.best_member_so_far.as_ref().unwrap().genome.objective_fitness.unwrap();
            let best_member_this_gen_fitness = self.run_context.get_best_member_in_this_gen.as_ref().unwrap().genome.objective_fitness.unwrap();
            if m.genome.objective_fitness.is_some() {
                let fitness =  m.genome.objective_fitness.unwrap();

                if self.run_context.worst_objective_fitness_so_far.is_none() || fitness < self.run_context.worst_objective_fitness_so_far.unwrap(){
                    self.run_context.worst_objective_fitness_so_far = Some(fitness);
                }

                if fitness > best_member_this_gen_fitness {
                    self.run_context.get_best_member_in_this_gen = Some(m.clone());
                }
               
                if fitness > best_member_ever_fitness {
                    self.send_event(FromHostEvent::BestNewGenome(m.clone()));
                    self.run_context.best_member_so_far = Some(m.clone());
                }
            }
        }
    }
    pub fn save(&mut self) -> std::io::Result<()>{
        let json = serde_json::to_string(&self).unwrap();
        let mut file = File::create(format!("{}{}-{}.neatrun", self.configuration.run_save_directory, self.configuration.run_name, self.get_current_generation()))?;
        file.write_all(json.as_bytes())
    }
    pub fn load(name:String) -> Option<NeatTrainer>{
        let mut json = String::new();
        let file_result = File::open(name);
        file_result.unwrap().read_to_string(&mut json).unwrap();
        let result = serde_json::from_str(&mut json);
        if result.is_ok(){
            return result.unwrap()
        }
        println!("Error Loading Saved Run: {}", result.err().unwrap());
        return None
    }
    fn send_event(&self, event: FromHostEvent){
        if self.event_sender.is_some(){
            let _ = self.event_sender.as_ref().unwrap().send(event);
        }
    }
    fn recompute_indexes(&mut self){
        
        self.members_map.clear();
        for i in 0..self.members.len(){
            let member = &self.members[i];
            self.members_map.insert(member.genome.id.as_u64_pair().0, i);
        }
    }

    pub fn set_config_regulators(&mut self, regulators: Vec<ConfigRegulator>) {
        self.config_regulators = regulators
    }
}
