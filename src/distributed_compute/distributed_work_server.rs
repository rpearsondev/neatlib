use std::net::SocketAddr;
use tarpc::context;

use crate::distributed_compute::queues::{CPU_DISTRIBUTED_WORK_QUEUE, CPU_DISTRIBUTED_RESULTS_QUEUE};

use super::models::{to_agent_work_item::ToAgentWorkItem, from_agent_work_result::FromAgentWorkResult, generation_data::GenerationData, to_agent_work_item_batch::ToAgentWorkItemBatch};

#[tarpc::service]
pub trait DistributedWork {
    async fn get_work(client_name: String, batch_size: usize) ->  ToAgentWorkItemBatch;
    async fn get_latest_generation_data(client_name: String) ->  GenerationData;
    async fn submit_results(client_name: String, results: Vec<FromAgentWorkResult>);
}

#[derive(Clone)]
pub struct DistributedWorkServer(pub SocketAddr);

#[tarpc::server]
impl DistributedWork for DistributedWorkServer {
    async fn get_work(self, _c: context::Context, _client_name: String, batch_size: usize) -> ToAgentWorkItemBatch {
        let mut queue = CPU_DISTRIBUTED_WORK_QUEUE.lock().unwrap();
        let mut work_for_agent:Vec<ToAgentWorkItem> =  Vec::with_capacity(batch_size);
        for _ in 0..batch_size{
            if queue.to_agent_queue.len() ==0{
                break;
            }
            let item_opt =queue.to_agent_queue.swap_remove(0);
            work_for_agent.push(item_opt);
        }
        return ToAgentWorkItemBatch{ members: work_for_agent, generation: queue.current_generation }
    }
    async fn get_latest_generation_data(self, _c: context::Context, _client_name: String) -> GenerationData {
        let queue = CPU_DISTRIBUTED_WORK_QUEUE.lock().unwrap();
        return GenerationData{
            generation: queue.current_generation,
            novelty_component_store: queue.latest_novelty_component_store.clone(),
            configuration: queue.latest_configuration.clone()
        }
    }
    async fn submit_results(self, _c: context::Context, _client_name: String, results: Vec<FromAgentWorkResult>){
        let mut queue = CPU_DISTRIBUTED_RESULTS_QUEUE.lock().unwrap();
        for result in results{
            queue.from_agent_results_queue.push(result)
        }
    }
}
