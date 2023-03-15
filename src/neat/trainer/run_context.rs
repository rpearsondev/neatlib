use crate::{neat::genetable::{connect_gene_table::ConnectGeneTable, node_gene_index::NoneGeneIndex}, neat::population::{GenerationMember}, neat::{genome::neat::{NeatGenome}, population::speciation::species::Species}, common::NeatFloat};
use hashbrown::{HashMap};
use serde::{Deserialize, Serialize};
use super::{seed_bank::SeedBank, fitness::novelty_component_store::NoveltyComponentStore, generation_stats::GenerationStats};

#[derive(Serialize, Deserialize, Clone)]
pub struct RunContext{
    pub gene_table: ConnectGeneTable,
    pub node_index: NoneGeneIndex,
    pub species_index: HashMap<uuid::Uuid, Species>,
    pub new_species_created_on_last_generation: u32,
    pub current_generation: u32,
    pub best_member_so_far: Option<GenerationMember<NeatGenome>>,
    pub get_best_member_in_this_gen: Option<GenerationMember<NeatGenome>>,
    pub worst_objective_fitness_so_far: Option<NeatFloat>,
    pub seed_bank: SeedBank<NeatGenome>,
    pub novelty_component_store: NoveltyComponentStore,
    pub last_ten_thousand_generations_stats: Vec<GenerationStats>
}

impl RunContext{
    pub fn new(initial_node_count: usize, seed_bank_limit: usize) -> Self{
        RunContext{
            gene_table: ConnectGeneTable::new(),
            node_index: NoneGeneIndex::new(initial_node_count),
            species_index: HashMap::new(),
            new_species_created_on_last_generation: 0,
            current_generation: 0,
            best_member_so_far: None,
            get_best_member_in_this_gen: None,
            worst_objective_fitness_so_far: None,
            seed_bank: SeedBank::new(seed_bank_limit),
            novelty_component_store: NoveltyComponentStore::new(),
            last_ten_thousand_generations_stats: Vec::new()
        }
    }
    pub fn increment_generation(&mut self){
        self.current_generation += 1;
        if self.current_generation > 2 {
            self.last_ten_thousand_generations_stats.push(GenerationStats::new(self));
            while self.last_ten_thousand_generations_stats.len() > 5_000 {
                self.last_ten_thousand_generations_stats.remove(0);
            }
        }
    }
    pub fn reset(&mut self){
        self.current_generation = 0;
        self.species_index.clear();
        self.node_index.clear();
        self.gene_table.reset();
        self.novelty_component_store.clear();
        self.last_ten_thousand_generations_stats.clear();
    }
}