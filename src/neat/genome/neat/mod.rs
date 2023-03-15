#[cfg(test)]
mod neat_genome_tests;

pub mod neat_genome;
pub mod neat_genes;
pub mod connect_gene;
pub mod connect_genes;
pub mod node_gene;
pub mod node_genes;
pub use neat_genome::NeatGenome as NeatGenome;
pub mod mutation;
pub mod mutation_mode;
pub mod mutation_add_mode;