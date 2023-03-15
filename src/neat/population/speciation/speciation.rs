use std::cmp::Ordering;
use bevy::prelude::Color;
use hashbrown::{HashMap as HashMap, HashSet};
use uuid::Uuid;
use rayon::prelude::*;
use rand::{Rng, seq::SliceRandom};
use serde::{Serialize, Deserialize};
use crate::{phenome::Phenome, common::{random::Random, NeatFloat, event_stream::{event::{EventType, Event}, event_recorder::EventRecorder}, types::NeatFloatExtensions}, neat::{population::{members_lookup::MembersLookup, GenerationMember, reproduction::Reproduction, speciation::{species_member::SpeciesMember, species_metric::SpeciesMetric, species_member_terminator::SpeciesMemberTerminator, genetically_engineered_member_maker::GeneticallyEngineeredMemberMaker}}, genome::{neat::{NeatGenome, mutation_mode::MutationMode}, genome::Genome}, trainer::{configuration::{Configuration, OffSpringMode}, run_context::RunContext}}};
use super::species::Species;
pub struct Speciation;

impl Speciation{
    pub fn speciate(mut members: &mut Vec<GenerationMember<NeatGenome>>, configuration: &Configuration, run_context: &mut RunContext) {
        if members.len() == 0 || run_context.current_generation == 0 {
            run_context.reset();
            members.clear();


            let population_size = configuration.population_size;
            for _ in 0..population_size{
                let mut new_genome = NeatGenome::minimal(configuration, run_context);
    
                new_genome.mutate(configuration, run_context, MutationMode::Optimistic);

                new_genome.objective_fitness = None;
                let member = GenerationMember::new(new_genome, run_context.current_generation);
                members.push(member)
            }

            if run_context.current_generation == 0 {
                Self::create_initial_species(members, configuration, run_context);
            }

            return;
        }
        run_context.new_species_created_on_last_generation = 0;
        Self::check_objective_fitness_set(members);
        Self::update_seed_bank(members, configuration, run_context);
        Self::clear_species_members_and_set_primary(members, configuration, run_context);
        Self::add_best_members_back_in(members, configuration, run_context);
        Self::put_members_in_existing_species_or_create_new(members, configuration, run_context);
        Self::calculate_species_fitness(&mut members, configuration, run_context);
        Self::remove_species_that_have_not_improved(configuration, run_context);
        Self::set_allowed_offspring(&mut members, configuration, run_context);
        Self::remove_species_with_no_offspring( configuration, run_context);
        Self::remove_members_of_species_below_threshold(configuration, run_context);
        Self::cross_species_reproduction(&mut members, configuration, run_context); 
        Self::produce_offspring_within_species(&mut members, configuration, run_context); 
        GeneticallyEngineeredMemberMaker::make_genetically_engineered_members_v1(members, configuration, run_context);
        Self::add_new_species_during_run_if_required(members, configuration, run_context);
        Self::remove_old_generation_members(&mut members, configuration, run_context);     
    }
    fn update_seed_bank(mut members: &mut Vec<GenerationMember<NeatGenome>>, configuration: &Configuration, run_context: &mut RunContext) {
        if configuration.speciation_use_best_seed_bank.is_none(){
            return;
        }
        run_context.seed_bank.update_seed_bank(members);
    }
    fn clear_species_members_and_set_primary(mut members: &mut Vec<GenerationMember<NeatGenome>>, configuration: &Configuration, run_context: &mut RunContext) {
        
        let members_map:HashMap<uuid::Uuid, &GenerationMember<NeatGenome>> = HashMap::from_iter(members.iter().map(|m| (m.genome.id, m)));
        let mut rng = rand::thread_rng();
        for (_ ,species) in run_context.species_index.iter_mut(){
            if species.members.len() == 0 {
                continue;
            }

            species.members.sort_unstable();
            let mut best_primary_species_member = (&species.members[0]).clone();
            best_primary_species_member.is_elite = true;
            let (_, new_primary_generation_member) = members_map.get_key_value(&best_primary_species_member.id).unwrap();
            let owned_primary_generation_member = (*new_primary_generation_member).clone();
            species.members.clear();
            species.primary = owned_primary_generation_member;
        }
    }
    fn add_new_species_during_run_if_required(members: &mut Vec<GenerationMember<NeatGenome>>, mut configuration: &Configuration, run_context: &mut RunContext){
        if !configuration.speciation_add_new_species_during_run{
            return;
        }
      
        if run_context.species_index.len() < configuration.target_species as usize{
            for i in 1..configuration.population_size{
                let mut new_genome = NeatGenome::minimal(configuration, run_context);
                new_genome.mutate(configuration, run_context, MutationMode::FoolHardy);
                new_genome.objective_fitness = None;
                members.push(GenerationMember::new(new_genome, run_context.current_generation))
            }
        }
    }
    fn create_initial_species(members: &mut Vec<GenerationMember<NeatGenome>>, mut configuration: &Configuration, run_context: &mut RunContext){
        
        let threshold = configuration.speciation_genetic_distance_threshold;
        let mut total_speciation_distance_approx = 0.0;
     
        let mr_one = &members[0];
        let mut species = &mut run_context.species_index;
        let members = members.iter().map(|m| SpeciesMember::new(m.genome.id, 0.0, 0.0, false)).collect::<Vec<SpeciesMember>>();
        let first_species = Species::new(
            uuid::Uuid::new_v4(),
            mr_one.clone(),
            members,
            run_context.current_generation,
            run_context.current_generation + configuration.speciation_new_species_protected_for_generations
        );
        species.insert(first_species.id, first_species);
    }
    fn put_members_in_existing_species_or_create_new(members: &mut Vec<GenerationMember<NeatGenome>>, mut configuration: &Configuration, run_context: &mut RunContext){
        
        let threshold = configuration.speciation_genetic_distance_threshold;
        
        for outer in members.iter_mut() {
            let mut species_number = 0;
            let mut species_found = false;

            let hint_species = run_context.species_index.get_mut(&outer.species_hint);
            if hint_species.is_some(){
                let hint_species_unwrapped = hint_species.unwrap();
                let genetic_distance = outer.genome.get_genetic_difference_distance_from(&hint_species_unwrapped.primary.genome, threshold);
                if(genetic_distance == NeatFloat::INFINITY || genetic_distance == NeatFloat::NAN || genetic_distance < 0.0){
                    panic!("genetic distance inf or nan");
                }
                
                if genetic_distance < threshold {
                        hint_species_unwrapped.members.push(SpeciesMember::new(outer.genome.id, outer.genome.objective_fitness.unwrap(), outer.genome.novelty, false ));
                        species_found = true;
                        continue;
                }
            }
            
            for (s, existing_species) in  run_context.species_index.iter_mut(){
                let genetic_distance = outer.genome.get_genetic_difference_distance_from(&existing_species.primary.genome, threshold);
                if(genetic_distance == NeatFloat::INFINITY || genetic_distance == NeatFloat::NAN ||genetic_distance < 0.0){
                    panic!("genetic distance inf or nan");
                }
                if genetic_distance < threshold {
                        existing_species.members.push(SpeciesMember::new(outer.genome.id, outer.genome.objective_fitness.unwrap(), outer.genome.novelty, false));
                        species_found = true;
                        outer.species_hint = s.clone();
                        break;
                }
                species_number+= 1;
            }

            if !species_found{
                let key = uuid::Uuid::new_v4();
                run_context.species_index.insert(key, 
                Species::new(key, (*outer).clone(),
                    vec![SpeciesMember::new(outer.genome.id, outer.genome.objective_fitness.unwrap(), outer.genome.novelty, false)],
                    run_context.current_generation,
                    run_context.current_generation + configuration.speciation_new_species_protected_for_generations
                ));
                run_context.new_species_created_on_last_generation += 1;
                outer.species_hint = key;
                if EventRecorder::has_subscription(configuration, EventType::SPECIATION_SPECIES_NEW){
                    EventRecorder::record_event(configuration, &Event::species_species_new(run_context.current_generation, &key));
                }
            }
        }
    }
    fn calculate_species_fitness(members: &mut Vec<GenerationMember<NeatGenome>>, configuration: &Configuration, run_context: &mut RunContext){
        let mut all_members_max_objective_fitness: Option<NeatFloat> = None;
        let mut all_members_min_objective_fitness: Option<NeatFloat> = None;
        let mut all_members_max_outcome_novelty: NeatFloat = 0.0;
        let mut all_members_min_outcome_novelty: NeatFloat = 0.0;
        for m in members.iter().map(|m| &m.genome){
            let objective_fitness = m.objective_fitness.unwrap();
            let outcome_novelty = m.novelty;
            if  all_members_max_objective_fitness.is_none() || objective_fitness > all_members_max_objective_fitness.unwrap() {
                all_members_max_objective_fitness = Some(objective_fitness);
            }
            if all_members_min_objective_fitness.is_none() || objective_fitness < all_members_min_objective_fitness.unwrap(){
                all_members_min_objective_fitness = Some(objective_fitness);
            }
            if outcome_novelty > all_members_max_outcome_novelty{
                all_members_max_outcome_novelty = outcome_novelty;
            }
            if outcome_novelty < all_members_min_outcome_novelty{
                all_members_min_outcome_novelty = outcome_novelty;
            }
        }
     
        let all_members_min_fitness = all_members_min_objective_fitness.unwrap();
        let worst_ever = run_context.worst_objective_fitness_so_far.unwrap_or(all_members_min_fitness);

        let mut min_species_avg_objective_fitness: NeatFloat = 0.0;
        let mut min_species_avg_novelty: NeatFloat = 0.0;
        let mut max_species_avg_objective_fitness: NeatFloat = 0.0;
        let mut max_species_avg_novelty: NeatFloat = 0.0;
        for (_, species) in run_context.species_index.iter_mut(){
            if members.len() == 0{
                species.objective_fitness = SpeciesMetric::new();
                species.outcome_novelty = SpeciesMetric::new();
                continue;
            }
            species.objective_fitness.new_generation(worst_ever, &species.members, |s| s.objective_fitness, run_context.current_generation);
            species.outcome_novelty.new_generation(0.0, &species.members, |s| s.outcome_novelty, run_context.current_generation);
            
            if species.objective_fitness.average < min_species_avg_objective_fitness{
                min_species_avg_objective_fitness = species.objective_fitness.average;
            }
            if species.outcome_novelty.average < min_species_avg_objective_fitness{
                min_species_avg_novelty = species.outcome_novelty.average
            }

            if species.objective_fitness.average > max_species_avg_objective_fitness{
                max_species_avg_objective_fitness = species.objective_fitness.average;
            }
            if species.outcome_novelty.average > max_species_avg_objective_fitness{
                max_species_avg_novelty = species.outcome_novelty.average
            }

            species.stagnant_generation_counter += 1;
            if species.stagnant_generation_counter >= configuration.speciation_remove_stagnant_species_generations{
                if species.objective_fitness.average < species.stagnant_objective_fitness && species.outcome_novelty.average < species.stagnant_novelty{
                    species.is_stagnant = true;
                }else{
                    species.stagnant_objective_fitness = species.objective_fitness.average;
                    species.stagnant_novelty = species.outcome_novelty.average;
                    species.stagnant_generation_counter = 0;
                }
            }
        }

        let objective_fitness_member_range = NeatFloat::max(NeatFloatExtensions::abs_diff(all_members_max_objective_fitness.unwrap(),all_members_min_fitness),1.0);
        let outcome_novelty_member_range = NeatFloat::max(NeatFloatExtensions::abs_diff(all_members_max_outcome_novelty,all_members_min_outcome_novelty),1.0);
        let objective_fitness_species_range = NeatFloat::max(NeatFloatExtensions::abs_diff(max_species_avg_objective_fitness,min_species_avg_objective_fitness),1.0);
        let outcome_novelty_species_range = NeatFloat::max(NeatFloatExtensions::abs_diff(max_species_avg_novelty,min_species_avg_novelty),1.0);

        for (_, species) in run_context.species_index.iter_mut(){
            species.adjusted_average_objective_fitness_based_on_member_range = (species.objective_fitness.average - all_members_min_fitness) / objective_fitness_species_range;
            species.adjusted_average_outcome_novelty_based_on_member_range = (species.outcome_novelty.average - all_members_min_outcome_novelty) / outcome_novelty_species_range;
            species.adjusted_average_objective_fitness_based_on_species_range = (species.objective_fitness.average - min_species_avg_objective_fitness) / objective_fitness_species_range;
            species.adjusted_average_outcome_novelty_based_on_species_range = (species.outcome_novelty.average - min_species_avg_novelty) / outcome_novelty_species_range;
        }
    }
    fn remove_species_that_have_not_improved(configuration: &Configuration, run_context: &mut RunContext) {
        
        run_context.species_index.retain(|k,species| { 
            let is_still_protected = species.species_protected_until_generation > run_context.current_generation;
            let has_objective_fitness_improved_in_last_x_generations = (run_context.current_generation - species.objective_fitness.last_generation_improved) < configuration.speciation_drop_species_no_improvement_generations;
            let has_outcome_novelty_improved_in_last_x_generations = (run_context.current_generation - species.outcome_novelty.last_generation_improved) < configuration.speciation_drop_species_no_improvement_generations;
            let species_is_not_stagnant = !species.is_stagnant;
            let has_members = species.members.len() > 0;
            let allowed_to_live = has_members && (is_still_protected || ((has_objective_fitness_improved_in_last_x_generations || has_outcome_novelty_improved_in_last_x_generations) && species_is_not_stagnant));

            if !allowed_to_live{
                if EventRecorder::has_subscription(configuration, EventType::SPECIATION_SPECIES_REMOVE){
                    EventRecorder::record_event(configuration, &Event::species_species_remove_no_improvement(run_context.current_generation, k));
                }
            }

            allowed_to_live
        });
    }
    fn remove_species_with_no_offspring(configuration: &Configuration, run_context: &mut RunContext) {
        
        run_context.species_index.retain(|k,species| { 
            let has_offspring = species.allowed_number_of_offspring_based_on_objective_fitness > 2.0
                                        || species.allowed_number_of_offspring_based_on_outcome_novelty > 2.0;
            let is_protected = species.species_protected_until_generation < run_context.current_generation;
            let is_retained = has_offspring || is_protected;
            
            if !is_retained{
                if EventRecorder::has_subscription(configuration, EventType::SPECIATION_SPECIES_REMOVE){
                    EventRecorder::record_event(configuration, &Event::species_species_remove_no_offspring(run_context.current_generation, k));
                }
            }else{
                if !has_offspring{
                    // if they are protected, then give them a meaningful amount of members
                    species.allowed_number_of_offspring_based_on_objective_fitness = 5.0;
                    species.allowed_number_of_offspring_based_on_outcome_novelty = 5.0;
                }
            }
            
            is_retained
        });

        let sum_of_remaining_offspring_for_fitness = run_context.species_index.iter().map(|(_ ,s)| s.allowed_number_of_offspring_based_on_objective_fitness).sum::<NeatFloat>();
        let sum_of_remaining_offspring_for_novelty = run_context.species_index.iter().map(|(_ ,s)| s.allowed_number_of_offspring_based_on_outcome_novelty).sum::<NeatFloat>();

        let fitness_factor = sum_of_remaining_offspring_for_fitness / configuration.population_size as NeatFloat;
        let novetly_factor = sum_of_remaining_offspring_for_novelty / configuration.population_size as NeatFloat;

        for (_, species ) in run_context.species_index.iter_mut(){
            species.allowed_number_of_offspring_based_on_objective_fitness = species.allowed_number_of_offspring_based_on_objective_fitness * (1.0 / fitness_factor);
            species.allowed_number_of_offspring_based_on_outcome_novelty = species.allowed_number_of_offspring_based_on_outcome_novelty * (1.0 / novetly_factor);
        }

    }
    fn remove_members_of_species_below_threshold(configuration: &Configuration, run_context: &mut RunContext){
        
        for (_, species) in run_context.species_index.iter_mut(){
            if species.members.len() < configuration.speciation_species_min_number_of_members{
                continue;
            }
            SpeciesMemberTerminator::terminate_rejects(species, configuration, run_context.current_generation)
        }
    }
    fn check_objective_fitness_set(members: &mut Vec<GenerationMember<NeatGenome>>){
        
        let all_members_fitness_set = members.par_iter().all(|m| m.genome.objective_fitness != None);
        if !all_members_fitness_set{
            panic!("all members fitness must be set");
        }
    }
    fn remove_old_generation_members(members: &mut Vec<GenerationMember<NeatGenome>>, configuration: &Configuration, run_context: &mut RunContext) {
        members.retain(|m| m.created_generation == run_context.current_generation);
    }
    fn set_allowed_offspring(members: &mut Vec<GenerationMember<NeatGenome>>, configuration: &Configuration, run_context: &mut RunContext){
        
        let mut total_of_species_fitness: NeatFloat = 0.0;
        
        match configuration.speciation_offspring_mode {
            OffSpringMode::Average => total_of_species_fitness = run_context.species_index.iter().map(|(_, s)| s.objective_fitness.positively_adjusted_average).sum::<NeatFloat>(),
            OffSpringMode::AdjustedMemberRange =>  total_of_species_fitness = run_context.species_index.iter().map(|(_, s)| s.adjusted_average_objective_fitness_based_on_member_range).sum::<NeatFloat>(),
            OffSpringMode::AdjustedSpeciesRange =>  total_of_species_fitness = run_context.species_index.iter().map(|(_, s)| s.adjusted_average_objective_fitness_based_on_species_range).sum::<NeatFloat>(),

        }

        let mut total_of_outcome_novelty =  run_context.species_index.iter().map(|(_, s)| s.outcome_novelty.average).sum::<NeatFloat>();

        if total_of_outcome_novelty == 0.0{
            total_of_outcome_novelty = 1.0;
        }

        for (s, species) in run_context.species_index.iter_mut(){
            match configuration.speciation_offspring_mode {
                OffSpringMode::Average =>  {
                    species.allowed_number_of_offspring_based_on_objective_fitness = ((species.objective_fitness.positively_adjusted_average / total_of_species_fitness) * (configuration.population_size as NeatFloat));
                    species.allowed_number_of_offspring_based_on_outcome_novelty= ((species.outcome_novelty.average / total_of_outcome_novelty) * (configuration.population_size as NeatFloat));
                },
                OffSpringMode::AdjustedMemberRange => {
                    species.allowed_number_of_offspring_based_on_objective_fitness = ((species.adjusted_average_objective_fitness_based_on_member_range / total_of_species_fitness) * (configuration.population_size as NeatFloat));
                    species.allowed_number_of_offspring_based_on_outcome_novelty= ((species.adjusted_average_outcome_novelty_based_on_member_range / total_of_outcome_novelty) * (configuration.population_size as NeatFloat));
                },
                OffSpringMode::AdjustedSpeciesRange => {
                    species.allowed_number_of_offspring_based_on_objective_fitness = ((species.adjusted_average_objective_fitness_based_on_species_range / total_of_species_fitness) * (configuration.population_size as NeatFloat));
                    species.allowed_number_of_offspring_based_on_outcome_novelty= ((species.adjusted_average_outcome_novelty_based_on_species_range / total_of_outcome_novelty) * (configuration.population_size as NeatFloat));
                }
            }
            let too_many_members_based_on_fitness = (species.allowed_number_of_offspring_based_on_objective_fitness > configuration.population_size as NeatFloat);
            let too_many_members_based_on_novelty = (species.allowed_number_of_offspring_based_on_outcome_novelty > configuration.population_size as NeatFloat);

            if too_many_members_based_on_fitness || too_many_members_based_on_novelty{
                println!("DEBUG: Species has created too many members: {:?}", species);
                species.allowed_number_of_offspring_based_on_outcome_novelty = configuration.population_size as NeatFloat;
                species.allowed_number_of_offspring_based_on_objective_fitness = configuration.population_size as NeatFloat;
            }
            if total_of_outcome_novelty == 0.0{
                species.allowed_number_of_offspring_based_on_outcome_novelty = species.allowed_number_of_offspring_based_on_objective_fitness;
            }
        }
    }
    fn cross_species_reproduction (members: &mut Vec<GenerationMember<NeatGenome>>, mut configuration: &Configuration, run_context: &mut RunContext) {
        let mut number_of_cross_species_to_create = ((members.len() as f32 * configuration.speciation_cross_species_reproduction_scale)) as usize;

        if number_of_cross_species_to_create == 0{
            return;
        }
        let mut rng = rand::thread_rng();
        
        members.sort_by(|a, b| {b.genome.objective_fitness.unwrap().partial_cmp(&a.genome.objective_fitness.unwrap()).unwrap()});

        let mut try_count = 0;
        for i in 0..number_of_cross_species_to_create{
            try_count+= 1;
            if try_count > number_of_cross_species_to_create * 2{
                //So that we dont end up in a continuous loop, only try for twice as many cross species members that we want to create.
                break;
            }

            let first_member_index = Random::gen_range_usize(0, number_of_cross_species_to_create);
            let first_member = &members[first_member_index];
            let mut first_member_genome = first_member.genome.clone();
            
            let second_member_index = Random::gen_range_usize(0, number_of_cross_species_to_create);
            let second_member = &members[second_member_index];
            let second_member_genome = &second_member.genome;

            if first_member.species_hint == second_member.species_hint{
                number_of_cross_species_to_create += 1;
                continue;
            }

            let best_performing: &NeatGenome;
            let other: &NeatGenome;

            if first_member_genome.objective_fitness > second_member_genome.objective_fitness {
                best_performing = &first_member_genome;
                other =  second_member_genome;
            }else{
                best_performing = second_member_genome;
                other = &first_member_genome;
            }

            let new_genome = Reproduction::reproduce_cross_species(best_performing, other, first_member.species_hint, configuration.reproduction_weights_from_fitter_probability);

            if EventRecorder::has_subscription(configuration, EventType::SPECIATION_REPRODUCE_CROSS_SPECIES){
                EventRecorder::record_event(configuration, &Event::speciation_reproduce_cross_species(run_context.current_generation, best_performing, other, &new_genome, &first_member.species_hint));
            }

            let mut species_iter_mut = run_context.species_index.iter_mut();
            let first_species = species_iter_mut.find(|s| s.0 == &first_member.species_hint);
            let mut species_opt: Option<&mut Species> = None;
            if first_species.is_some(){
                species_opt = Some(first_species.unwrap().1)
            }else{
                let second_species = species_iter_mut.find(|s| s.0 == &second_member.species_hint);
                if second_species.is_some(){
                    species_opt = Some(second_species.unwrap().1)
                }else{
                    let any_species = species_iter_mut.last();
                    if any_species.is_some(){
                        species_opt = Some(any_species.unwrap().1)
                    }
                }
            }

            let mut species_hint = Uuid::nil();
            if species_opt.is_some(){
                let sp = species_opt.unwrap();
                sp.members.push(SpeciesMember::new(new_genome.id, 0.0, 0.0, true));
                species_hint = sp.id;
            }
            
            members.push(GenerationMember{
                genome: new_genome, 
                created_generation: run_context.current_generation,
                number_of_generations_since_species_improved: 0,
                species_hint: species_hint,
                hyperneat_network_definition: None
            });
        }

    }
    fn produce_offspring_within_species(members: &mut Vec<GenerationMember<NeatGenome>>, mut configuration: &Configuration, run_context: &mut RunContext) {
        let members_lookup = MembersLookup::new(members);

        let members_to_add = run_context.species_index.iter_mut().par_bridge().into_par_iter().map(|(s, species)| {
            optick::register_thread("Produce_Offspring par");
            let mut rng = rand::thread_rng(); 
            let mut old_members = species.members.clone();
            old_members.sort_unstable();
            species.members.retain(|m| m.is_cross_species );

            if species.allowed_number_of_offspring_based_on_objective_fitness == 0.0 && species.species_protected_until_generation < run_context.current_generation {
                return vec![];
            }

            let range_between_objective_fitness_and_outcome_novelty = species.allowed_number_of_offspring_based_on_outcome_novelty - species.allowed_number_of_offspring_based_on_objective_fitness;
            let balanced = species.allowed_number_of_offspring_based_on_objective_fitness + (range_between_objective_fitness_and_outcome_novelty * configuration.speciation_offspring_outcome_novelty_weight);
      
            let mut number_of_offspring = u32::max(NeatFloat::floor(balanced) as u32, configuration.speciation_species_min_number_of_members as u32);
            let mut members_to_add: Vec<(NeatGenome, uuid::Uuid, u32)> = Vec::with_capacity(number_of_offspring as usize);
            let number_of_generations_since_species_improved = run_context.current_generation - u32::min( species.outcome_novelty.last_generation_improved,species.objective_fitness.last_generation_improved);

            //elite members
            if configuration.speciation_preserve_elite && number_of_offspring > 1{
                number_of_offspring -= 1;

                let elite_species_member = &old_members[0];
                let elite_member = &members[members_lookup.get_array_index(elite_species_member.id)];
                let id = Uuid::new_v4();
                let mut genome = elite_member.genome.clone();
                genome.id = id;
                genome.dont_allow_mutation();
                species.members.push(SpeciesMember::new(genome.id, 0.0, 0.0, false));
                members_to_add.push((genome, *s, number_of_generations_since_species_improved));
            }

            //reproduction within species
            for i in 0..number_of_offspring {
                let first_member_index = Random::gen_range_usize(0, old_members.len());
                let first_member_id = old_members[first_member_index].id;
                let first_member = &members[members_lookup.get_array_index(first_member_id)];
                let first_member_genome = &first_member.genome;
                
                let second_member_index = Random::gen_range_usize(0, old_members.len());
                let second_member_id = old_members[second_member_index].id;
                let second_member = &members[members_lookup.get_array_index(second_member_id)];
                let second_member_genome = &second_member.genome;

                let best_performing: &NeatGenome;
                let other: &NeatGenome;

                if first_member_genome.objective_fitness > second_member_genome.objective_fitness {
                    best_performing = first_member_genome;
                    other =  second_member_genome;
                }else{
                    best_performing = second_member_genome;
                    other = first_member_genome;
                }

                let new_genome = Reproduction::reproduce(best_performing, other, first_member.species_hint, configuration.reproduction_weights_from_fitter_probability);
                
                if EventRecorder::has_subscription(configuration, EventType::SPECIATION_REPRODUCE){
                    EventRecorder::record_event(configuration, &Event::speciation_reproduce(run_context.current_generation, best_performing, other, &new_genome, &species.id));
                }
                
                species.members.push(SpeciesMember::new(new_genome.id, 0.0, 0.0, false));
                members_to_add.push((new_genome, *s, number_of_generations_since_species_improved));
            }
            members_to_add
        }).flatten_iter().collect::<Vec<(NeatGenome, Uuid, u32)>>();

        let mut generation_members_to_add = members_to_add.into_iter().map(|(genome,species_hint, number_of_generations_since_species_improved)|{
            GenerationMember{
                genome: genome, 
                created_generation: run_context.current_generation,
                number_of_generations_since_species_improved,
                species_hint,
                hyperneat_network_definition: None
            }
        })
        .collect::<Vec<GenerationMember<NeatGenome>>>();


        
        members.append(&mut generation_members_to_add);
        
    }
    fn add_best_members_back_in(members: &mut Vec<GenerationMember<NeatGenome>>, mut configuration: &Configuration, run_context: &mut RunContext){
        if configuration.speciation_add_best_member_back_in{
            if run_context.best_member_so_far.is_some(){
                let best = run_context.best_member_so_far.as_mut().unwrap();
                best.genome.allow_mutation = false;
                members.push(GenerationMember::new(best.genome.clone(), run_context.current_generation));
            }
            if run_context.get_best_member_in_this_gen.is_some(){
                let best = run_context.get_best_member_in_this_gen.as_mut().unwrap();
                best.genome.allow_mutation = false;
                members.push(GenerationMember::new(best.genome.clone(), run_context.current_generation));
            }
        }
        if configuration.speciation_use_best_seed_bank.is_some(){
            for seed in &run_context.seed_bank.seeds{
                let mut seed = seed.clone();
                seed.genome.allow_mutation = false;
                members.push(GenerationMember::new(seed.genome, run_context.current_generation));
            }
        }
    }
}
