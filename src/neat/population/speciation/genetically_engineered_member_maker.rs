use uuid::Uuid;

use crate::{neat::{trainer::{configuration::Configuration, run_context::RunContext}, population::{GenerationMember, members_lookup::MembersLookup}, genome::neat::{NeatGenome, mutation::Mutation}}, common::NeatFloat};

pub struct GeneticallyEngineeredMemberMaker;
pub struct MutationGroup{
    pub gene_identity: u64,
    pub avg_fitness_delta: NeatFloat,
    pub parameter_values: Vec<MutationWithDelta>
}
pub struct MutationWithDelta{
    pub mutation: Mutation,
    pub fitness_delta: NeatFloat
}

impl GeneticallyEngineeredMemberMaker{
    pub fn make_genetically_engineered_members_v1(members: &mut Vec<GenerationMember<NeatGenome>>, mut configuration: &Configuration, run_context: &mut RunContext){
        let members_lookup = MembersLookup::new(members);
        
        for (species_id, species )in run_context.species_index.iter_mut(){

            let mut best_member: Option<&NeatGenome> = None;
            let mut mutations_for_members_where_fitness_improved: Vec<(NeatFloat, Mutation)> = Vec::new();
            for member in species.members.iter_mut(){
                let member = &members[members_lookup.get_array_index(member.id)];
                if member.genome.objective_fitness.is_none() || member.genome.parents_objective_fitness.is_none(){
                    continue;
                }

                //keep track of the best member
                if best_member.is_none() || (member.genome.objective_fitness.unwrap() > best_member.unwrap().objective_fitness.unwrap()) {
                    best_member = Some(&member.genome);
                }
                
                //add mutations along with fitness delta to list
                let objective_fitness = member.genome.objective_fitness.unwrap();
                let parents_objective_fitness = member.genome.parents_objective_fitness.unwrap();
                if objective_fitness > parents_objective_fitness{
                    let parent_child_fitness_delta = objective_fitness - parents_objective_fitness;
                    let mutations =  member.genome.mutations.iter().map(|m| (parent_child_fitness_delta, m.clone()))
                    .collect::<Vec<(NeatFloat, Mutation)>>();
                    for m in mutations {
                        mutations_for_members_where_fitness_improved.push(m);
                    }
                }
            }
            //group mutations
            let mut mutations_grouped: Vec<MutationGroup> = Vec::new();
            for (delta, mutation) in mutations_for_members_where_fitness_improved{
                let mutation_gene_identity = Mutation::get_identity(mutation);
                let value_opt = Mutation::get_value(mutation);

                if value_opt.is_none(){
                    continue;
                }
                let value = value_opt.unwrap();

                let existing_group_opt = mutations_grouped.iter_mut().find(|g| g.gene_identity == mutation_gene_identity);
                if existing_group_opt.is_none(){
                    mutations_grouped.push(MutationGroup { gene_identity: mutation_gene_identity, avg_fitness_delta: delta, parameter_values: vec![MutationWithDelta{fitness_delta: delta, mutation: mutation}] })
                }else{
                    let existing_group = existing_group_opt.unwrap();
                    existing_group.parameter_values.push(MutationWithDelta{fitness_delta: delta, mutation: mutation});
                    existing_group.avg_fitness_delta = existing_group.parameter_values.iter().map(|pv| pv.fitness_delta).sum::<NeatFloat>() / existing_group.parameter_values.len() as NeatFloat;
                }
            }

            mutations_grouped.sort_by(|a, b| b.avg_fitness_delta.partial_cmp(&a.avg_fitness_delta).unwrap());

            if best_member.is_some(){
                let mut mutant_genome = best_member.unwrap().clone();
                mutant_genome.id = uuid::Uuid::new_v4();

                let total_grouped_mutations = mutations_grouped.len();
                for mutation_group in mutations_grouped.iter_mut().take(total_grouped_mutations / 2) {
                    mutation_group.parameter_values.sort_by(|a, b| b.fitness_delta.partial_cmp(&a.fitness_delta).unwrap());
                    let top_mutation= &mutation_group.parameter_values[0];

                    Mutation::apply_optimization_to_genome(top_mutation.mutation, &mut mutant_genome);
                }
                members.push(GenerationMember{ 
                    genome: mutant_genome, 
                    created_generation: run_context.current_generation,
                    number_of_generations_since_species_improved: 0,
                    species_hint: species_id.clone(), 
                    hyperneat_network_definition: None });
            }
        }
    }

}