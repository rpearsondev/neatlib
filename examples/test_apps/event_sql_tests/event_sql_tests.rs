use neatlib::{common::event_stream::{listeners::sql_listener::SqlRepositoryEventListener, event::Event}, neat::trainer::run_context::RunContext};

pub fn main(){
    let run_context = RunContext::new(2,2);
    let genome_id = uuid::Uuid::new_v4();
    SqlRepositoryEventListener::process(&Event::mutation_connection_add(
         &run_context, 
         &genome_id, 
         1, 2));

    let species_id = uuid::Uuid::new_v4();
    SqlRepositoryEventListener::process(&Event::species_species_new(
            2, 
            &species_id));

    SqlRepositoryEventListener::print_tables();
}