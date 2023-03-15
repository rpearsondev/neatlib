use core::time;
use std::{net::{SocketAddr}, future, env};
use tarpc::{client, context, server::{self, Channel}};
use tokio::{runtime};
use futures::StreamExt;
use tarpc::server::incoming::Incoming;
use gethostname::gethostname;

use crate::{neat::{trainer::{fitness::{fitness_resolver::FitnessResolver}, activation_strategies::cpu_parallel::compute_fitnesses_cpu}, genome::{neat::NeatGenome}, population::GenerationMember}, phenome::Phenome, distributed_compute::{distributed_work_server::DistributedWorkClient, models::{from_agent_work_result::FromAgentWorkResult, generation_data::GenerationData}}, common::random::Random};

use super::{distributed_work_server::{DistributedWorkServer, DistributedWork}, distributed_config::DistributedConfig};

#[derive(Clone)]
pub struct DistributedAgent{
    pub host_address: SocketAddr,
    pub is_host: bool,
    pub config: DistributedConfig
}

impl DistributedAgent{
    pub fn new(host_address: SocketAddr, is_host: bool, config: DistributedConfig) -> Self{
        DistributedAgent{
            host_address,
            is_host,
            config
        }
    }
    pub fn start_service_host(&self){
        env::set_var("OTEL_LOG_LEVEL", "ERROR");
        env::set_var("OTEL_BSP_MAX_EXPORT_BATCH_SIZE", "0");
        env::set_var("OTEL_SDK_DISABLED", "true");
        env::set_var("OTEL_PROPAGATORS", "none");
        
        let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(self.config.host_thread_stack_size)
        .worker_threads(self.config.host_worker_threads)
        .max_blocking_threads(self.config.host_max_blocking_threads)
        .build().unwrap();

        rt.block_on(async move {
            let address = self.host_address.clone();
            println!("Starting listener on {}", address);
            let mut listener = tarpc::serde_transport::tcp::listen((address.ip(),address.port()), tarpc::tokio_serde::formats::Bincode::default).await.unwrap();
            listener.config_mut().max_frame_length(usize::MAX);
            listener
                // Ignore accept errors.
                .filter_map(|r| future::ready(r.ok()))
                .map(server::BaseChannel::with_defaults)
                // Limit channels to 1 per IP.
                .max_channels_per_key(50, |t| t.transport().peer_addr().unwrap().ip())
                // serve is generated by the service attribute. It takes as input any type implementing
                // the generated World trait.
                .map(|channel| {
                    let addr = channel.transport().peer_addr().unwrap();
                    let server = DistributedWorkServer(addr);
                    println!("Starting server on {}", addr);
                    channel.execute(server.serve())
                })
                .buffer_unordered(self.config.host_buffer_unordered)
                .for_each(|_| async {})
                .await;
                });
    }
    pub fn start_client_poller<F>(&self, fitness: &F) where F:Fn(&Phenome, &mut FitnessResolver) + std::marker::Sync{
        let address = self.host_address.clone();
        let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(self.config.client_thread_stack_size)
        .worker_threads(self.config.client_worker_threads)
        .max_blocking_threads(self.config.client_max_blocking_threads)
        .build().unwrap();
        rt.block_on(async move {
            loop{
                let transport = tarpc::serde_transport::tcp::connect((address.ip(),address.port()),
                tarpc::tokio_serde::formats::Bincode::default).await;

                if transport.as_ref().is_err() {
                    std::thread::sleep(time::Duration::from_millis(self.config.client_sleep_when_connection_dead_milliseconds));
                    println!("Waiting to establish connection to {}, error: {}", address, transport.as_ref().err().unwrap());
                    continue;
                }

                println!("Connection established to {}", address);

                let client = DistributedWorkClient::new(client::Config::default(),transport.unwrap() ).spawn();
                let host_name = gethostname().to_str().unwrap().to_owned();
                let mut work_is_empty_count = 0;
                let mut current_generation_data: Option<GenerationData> = None;
                loop{
                    let c = context::current();
                    println!("Requesting work...");
                    
                    let work_get_result = client.get_work(c, host_name.clone(), self.config.client_work_batch_size).await;
                    if work_get_result.is_err(){
                        println!("Failed to get work. error:{}", work_get_result.err().unwrap());
                        std::thread::sleep(time::Duration::from_millis(self.config.client_sleep_when_get_work_failed_milliseconds));
                        break;
                    }
                    
                    let work = work_get_result.unwrap();

                    if current_generation_data.as_ref().is_none() || current_generation_data.as_ref().unwrap().generation != work.generation{
                        let generation_data_result = client.get_latest_generation_data(c, host_name.clone()).await;

                        if generation_data_result.is_err(){
                            println!("Failed to get generation data. error:{}", generation_data_result.err().unwrap());
                            break;
                        }
    
                        current_generation_data = Some(generation_data_result.to_owned().unwrap());
                        
                        if current_generation_data.as_ref().unwrap().configuration.is_none(){
                            println!("No configuration");
                            std::thread::sleep(time::Duration::from_millis(self.config.client_sleep_when_no_config_milliseconds));
                            continue;
                        }
    
                        if current_generation_data.as_ref().unwrap().novelty_component_store.is_none(){
                            println!("No Novelty Store");
                            std::thread::sleep(time::Duration::from_millis(self.config.client_sleep_when_no_config_milliseconds));
                            continue;
                        }
                        println!("Moved to generation {}", current_generation_data.as_ref().unwrap().generation);
                    }


                    if work.members.is_empty(){
                        println!("No work available, waiting");
                        work_is_empty_count += 1;
                        if work_is_empty_count < 20 {
                            std::thread::sleep(time::Duration::from_millis(Random::gen_range_usize(self.config.client_sleep_when_no_work_milliseconds_low, self.config.client_sleep_when_no_work_milliseconds_high) as u64));
                        }
                        else if work_is_empty_count >= 100 {
                            std::thread::sleep(time::Duration::from_millis(self.config.client_sleep_when_no_work_after_100_attempts_milliseconds));
                        }
                        continue;
                    }else{
                        work_is_empty_count = 0;
                        println!("Got {} items of work to process.",  work.members.len());
                    }

                    let mut members = work.members.iter().map(|work_item| work_item.member.to_owned()).collect::<Vec<GenerationMember<NeatGenome>>>();
                    let computation_results = compute_fitnesses_cpu(fitness, current_generation_data.as_ref().unwrap().configuration.as_ref().unwrap(), &mut members, current_generation_data.as_ref().unwrap().novelty_component_store.as_ref().unwrap());

                    if computation_results.len() > 0 {
                        println!("Submitting {} results.",  computation_results.len());

                        let submission = computation_results.iter().map(|(id, fitness)| FromAgentWorkResult{genome_id: *id, fitness: fitness.clone()}).collect::<Vec<FromAgentWorkResult>>();

                        let submit_response = client.submit_results(c, host_name.clone(), submission).await;
                        if submit_response.is_err() {
                            println!("Error submitting results. error:{}",  submit_response.err().unwrap());
                        }
                    }else{
                        println!("No results to submit.");
                    }
                }
            }
        });
    }
}
