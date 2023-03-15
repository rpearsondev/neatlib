use std::thread::{self, JoinHandle};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;
use crate::common::event_stream::listeners::sql_listener::{SqlRepositoryEventListener};
use crate::neat::trainer::config_regulators::config_regulator::ConfigRegulator;
use crate::neat::trainer::configuration::Configuration;
use crate::neat::trainer::neat_trainer::NeatTrainer;
use super::from_host_events::FromHostEvent;
use super::models::run_stats::RunStats;
use super::neat_trainer_host_state::NeatTrainerHostState;
use super::to_host_events::ToHostEvents;

pub struct NeatTrainerHostClient{
    pub from_host_receiver: Receiver<FromHostEvent>,
    pub to_host_sender: Sender<ToHostEvents>
}
pub struct NeatTrainerHost{
    stop_host_receiver: Receiver<u8>,
    runner_join_handle: JoinHandle<()>,
    pub initial_configuration: Configuration,
    pub initial_config_regulators: Vec<ConfigRegulator>
}
impl NeatTrainerHost{
    pub fn new<F>(trainer: NeatTrainer, generation_func: F) -> (Self, NeatTrainerHostClient) where F:Fn(&mut NeatTrainer) + Send + Sync + 'static {
        let initial_configuration = trainer.configuration.clone();
        let initial_config_regulators = trainer.config_regulators.clone();
        let (to_client_tx, from_host_rx) = channel::<FromHostEvent>();
        let (to_host_tx, to_host_receiver) = channel::<ToHostEvents>();
        let (tx_stop, stop_host_receiver) = channel::<u8>();

        let to_client_tx_clone = to_client_tx.clone();
        let runner_join_handle = thread::spawn(move ||{
        
            let mut trainer = trainer;
            trainer.event_sender = Some(to_client_tx.clone());
            let to_host_receiver: Receiver<ToHostEvents> = to_host_receiver;
            let mut runner_state = NeatTrainerHostState::new();
            loop {
                Self::process_to_host_events(&mut runner_state, &mut trainer, &to_host_receiver, &tx_stop, &to_client_tx_clone);
                if runner_state.is_stopping{
                    break;
                }

                if runner_state.is_sleeping{
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }

                generation_func(&mut trainer);

                if trainer.has_met_success(){
                    runner_state.is_sleeping = true;
                    let _ = to_client_tx.send(FromHostEvent::HitSuccessThreshold(trainer.get_current_generation()));
                }
            }
        });
        (
            Self{
                initial_configuration,
                initial_config_regulators,
                stop_host_receiver,
                runner_join_handle
            },
            NeatTrainerHostClient{
                from_host_receiver: from_host_rx,
                to_host_sender: to_host_tx
            }
    )
    }
    pub fn block_until_stop(self){
        let _ = self.stop_host_receiver.recv();
        let _ = self.runner_join_handle.join();
        println!("Host stopped.")
    }
    fn process_to_host_events(host_state: &mut NeatTrainerHostState, trainer: &mut NeatTrainer, receiver: &Receiver<ToHostEvents>, tx_stop: &Sender<u8>, to_client_tx: &Sender<FromHostEvent>) {
        while let Ok(received) = receiver.try_recv(){
            match received{
                ToHostEvents::UpdateConfig(config) => {trainer.configuration = config;},
                ToHostEvents::Stop() => {
                    let _ = tx_stop.send(0); host_state.is_stopping = true;
                },
                ToHostEvents::SetRunUntil(gen) => {
                    if gen.is_none() || trainer.get_current_generation() < gen.unwrap(){
                        host_state.is_sleeping = false;
                    }else{
                        host_state.is_sleeping = true;
                    }
                },
                ToHostEvents::RequestRunStats() => {
                    let _ = to_client_tx.send(FromHostEvent::RunStats(RunStats::new(trainer)));
                }
                ToHostEvents::RequestLatestConfig() => {
                    let _ = to_client_tx.send(FromHostEvent::ConfigUpdate(trainer.configuration.clone()));
                },
                ToHostEvents::Reset() => {
                    trainer.reset();
                    SqlRepositoryEventListener::reset();
                },
                ToHostEvents::UpdateConfigRegulators(regulators) => {
                    trainer.set_config_regulators(regulators);
                }
                ToHostEvents::GenericOperation(request) => {
                    match request {
                        super::models::generic_operation::GenericOperation::Load(name) => {
                            let trainer_opt = NeatTrainer::load(name);
                            if trainer_opt.is_some(){
                                *trainer = trainer_opt.unwrap();
                                trainer.event_sender = Some(to_client_tx.clone());
                                let _ = to_client_tx.send(FromHostEvent::RunStats(RunStats::new(trainer)));
                                let _ = to_client_tx.send(FromHostEvent::ConfigUpdate(trainer.configuration.clone()));
                                let _ = to_client_tx.send(FromHostEvent::SetRunUntil(trainer.get_current_generation()));
                                let _ = to_client_tx.send(FromHostEvent::GenerationChange(trainer.get_current_generation()));
                                if trainer.get_best_member_so_far().is_some(){
                                    let _ = to_client_tx.send(FromHostEvent::BestNewGenome(trainer.get_best_member_so_far().as_ref().unwrap().clone()));
                                }
                                
                            }
                        },
                        super::models::generic_operation::GenericOperation::Save() => {
                            let result = trainer.save();
                            if result.is_err(){
                                println!("File Save Error: {}", result.err().unwrap());
                            }
                        },
                    }
                
                },
            }
        }
    }
}


