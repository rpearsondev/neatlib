use crate::{neat::trainer::configuration::Configuration, common::{event_stream::{event::{EventType, Event}, event_recorder::EventRecorder}, NeatFloat}};
use rayon::prelude::*;
use super::{species::Species, species_member::SpeciesMember};
use itertools::Itertools;

pub struct SpeciesMemberTerminator;

impl SpeciesMemberTerminator{
    pub fn terminate_rejects(species: &mut Species, configuration: &Configuration, current_generation: u32){
         
        let mut sorted_by_fitness = species.members.clone();
         sorted_by_fitness.sort_by(|a, b| a.objective_fitness.partial_cmp(&b.objective_fitness).unwrap());

         let mut sorted_by_novelty = species.members.clone();
         sorted_by_novelty.sort_by(|a, b| a.outcome_novelty.partial_cmp(&b.outcome_novelty).unwrap());

         let mut combined_by_both: Vec<(usize, usize, &SpeciesMember)> = Vec::new();

        for (index, member) in sorted_by_fitness.iter().enumerate(){
            combined_by_both.push((index, 0, member))
        }

        for (index, member) in sorted_by_novelty.iter().enumerate(){
            let member = combined_by_both.iter_mut().find(|(_,_,m)| m.id == member.id).unwrap();
            member.1 = index;
        }

        let nov_weight = configuration.speciation_offspring_outcome_novelty_weight;
        combined_by_both.sort_by(|(a_fitness_index, a_novelty_index, _), (b_fitness_index, b_novelty_index, _)| {
            let a_combined =
                (*a_fitness_index as NeatFloat) //not scaling, as favouring fitness
                + ( (*a_novelty_index as NeatFloat) * nov_weight);
            let b_combined =
                (*b_fitness_index as NeatFloat) //not scaling, as favouring fitness
            + ( (*b_novelty_index as NeatFloat) * nov_weight);

            b_combined.partial_cmp(&a_combined).unwrap()
        });

        let cut_off_member_index = usize::max(1, f64::ceil((species.members.len() as f64) * (configuration.survival_threshold as f64)) as usize);

        species.members = combined_by_both.iter().take(cut_off_member_index).map(|(_,_,m)| (*m).clone()).collect::<Vec<SpeciesMember>>();

        for survivor in &species.members{
            if EventRecorder::has_subscription(configuration, EventType::SPECIATION_SURVIVOR){
                EventRecorder::record_event(configuration, &Event::species_survivor(current_generation, &species.id, &survivor));
            }
        }

        if species.members.len() ==0 {
            panic!("no members after terminating rejects")
        }
    }
}