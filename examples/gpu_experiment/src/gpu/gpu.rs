// use std::borrow::Cow;
// use wgpu::{ShaderModule};
// use wgpu::util::DeviceExt;

// use super::combined_network::{CombinedNetwork, Node};


// #[derive(Debug)]
// pub struct Gpu{
//     device: wgpu::Device,
//     queue: wgpu::Queue,
//     compute_layer_shader_module: Option<ShaderModule>
// }

// impl Gpu{
//     pub async fn new(preferred_adapter: String) -> Option<Self>{
//         let instance = wgpu::Instance::new(wgpu::Backends::all());
 
//         let mut adapter = instance
//             .request_adapter(&wgpu::RequestAdapterOptions::default())
//             .await?;
    
//             let adapters = instance.enumerate_adapters(wgpu::Backends::PRIMARY);
//             for a in adapters{
//                 let name = a.get_info().name;
//                 if name.contains(preferred_adapter.as_str()){
//                     adapter = a;
//                 }
//             }

//             println!("Device Chosen:{:?}", adapter.get_info().name);
    
//         let (device, queue) = adapter
//             .request_device(
//                 &wgpu::DeviceDescriptor {
//                     label: None,
//                     features: wgpu::Features::empty(),
//                     limits: wgpu::Limits::default(),
//                 },
//                 None,
//             )
//             .await
//             .unwrap();

//             Some(
//                 Gpu{
//                     device,
//                     queue,
//                     compute_layer_shader_module: None
//                 }
//             )
//     }
//     pub fn list_adapters() -> Vec<String>{
//         let instance = wgpu::Instance::new(wgpu::Backends::all()); 
//         let adapters = instance.enumerate_adapters(wgpu::Backends::PRIMARY);
//         let mut results: Vec<String> = Vec::new();
//         for a in adapters{
//             results.push(a.get_info().name);
//         }
//         results
//     }    
//     pub async fn compute_combined_network(&self, combined_network: &mut CombinedNetwork){ 
//         self.compute_combined(combined_network).await.unwrap();
        
//         for network in combined_network.network_descriptors.iter_mut() {
//             for output_index in 0..network.output_addresses.len(){
//                 let address = network.output_addresses[output_index];
//                 let mut val = combined_network.state[address];
//                 if val.is_nan(){
//                     //todo: need to resolve why outputs can be nan.
//                     //panic!("output is nan");
//                     val = 0.0;
//                 }
//                 network.outputs[output_index] = val;
//             }
//         }
//     }
//     pub fn setup_compute_layer_shader_module(&mut self) {
//         self.compute_layer_shader_module = Some(self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
//             label: None,
//             source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("compute_network_invocations.wgsl"))),
//         }));
//     }
//     async fn compute_combined<'a>(&self, combined_network: &mut CombinedNetwork) -> Result<(),()> {          
//         if combined_network.connections.len() == 0 {
//             return Ok(());
//         }

//         let widest_layer = combined_network.layers.iter().map(|l| l.length).max().unwrap_or_default();
        
//         let state_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//             label: Some("state"),
//             contents: bytemuck::cast_slice(&combined_network.state),
//             usage: wgpu::BufferUsages::STORAGE
//             | wgpu::BufferUsages::MAP_READ 
//             | wgpu::BufferUsages::COPY_SRC
//         });
        
//         let all_nodes_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//             label: None,
//             contents: bytemuck::cast_slice(&combined_network.nodes),
//             usage: wgpu::BufferUsages::STORAGE
//                 | wgpu::BufferUsages::COPY_SRC,
//         });

//         let current_nodes_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
//             label: None,
//             size: (widest_layer * std::mem::size_of::<Node>()) as wgpu::BufferAddress,
//             mapped_at_creation: false,
//             usage: wgpu::BufferUsages::STORAGE
//                 | wgpu::BufferUsages::MAP_READ 
//                 | wgpu::BufferUsages::COPY_DST,
//         });
            
//         let connection_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//             label: None,
//             contents: bytemuck::cast_slice(&combined_network.connections),
//             usage: wgpu::BufferUsages::STORAGE
//                 | wgpu::BufferUsages::COPY_DST,
//         });

//          let compute_layer_pipeline = self.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
//             label: None,
//             layout: None,
//             module: self.compute_layer_shader_module.as_ref().unwrap(),
//             entry_point: "main",
//         });
        
//         let bind_group_layout = compute_layer_pipeline.get_bind_group_layout(0);
//         let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
//             label: None,
//             layout: &bind_group_layout,
//             entries: &[
//                 wgpu::BindGroupEntry {
//                     binding: 0,
//                     resource: state_buffer.as_entire_binding(),
//                 },
//                 wgpu::BindGroupEntry {
//                     binding: 1,
//                     resource: current_nodes_buffer.as_entire_binding(),
//                 },
//                 wgpu::BindGroupEntry {
//                     binding: 2,
//                     resource: all_nodes_buffer.as_entire_binding(),
//                 },
//                 wgpu::BindGroupEntry {
//                     binding: 3,
//                     resource: connection_buffer.as_entire_binding(),
//                 }
//             ]
//         });

//     let mut encoder =  self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
//     for layer in &combined_network.layers{
        
//         encoder.clear_buffer(&current_nodes_buffer, 0, None);
//         let copy_size =  (layer.length * std::mem::size_of::<Node>()) as wgpu::BufferAddress;
//         let start = (layer.start_index  * std::mem::size_of::<Node>()) as wgpu::BufferAddress;
//         encoder.copy_buffer_to_buffer(&all_nodes_buffer,start, &current_nodes_buffer, 0, copy_size);

//         {
//             let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
//             compute_pass.set_pipeline(&compute_layer_pipeline);
//             compute_pass.set_bind_group(0, &bind_group, &[]);
//             compute_pass.insert_debug_marker("compute layer");
//             compute_pass.dispatch_workgroups((layer.length as f32 / 32 as f32).ceil() as u32, 1, 1); 
//         }
//     }

//     self.queue.submit(Some(encoder.finish()));

//     let buffer_slice = state_buffer.slice(..);

//     let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
//     buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    
//     self.device.poll(wgpu::Maintain::Wait);

//     if let Some(Ok(())) = receiver.receive().await {
//         let data = buffer_slice.get_mapped_range();
//         let state_after_run: Vec<f32> = bytemuck::cast_slice(&data).to_vec();

//         drop(data);
//         state_buffer.unmap(); 
//         combined_network.state = state_after_run;
//         return Ok(());
//     }else {
//         panic!("failed to run compute on gpu!")
//     }
//     }
// }


// #[cfg(test)]
// mod tests{
//     use neatlib::{node_kind::NodeKind, neat::{trainer::{configuration::Configuration, run_context::RunContext}, genome::{neat::{node_gene::NodeGene, NeatGenome, connect_gene::ConnectGene}, genome::Genome}}, activation_functions::ActivationFunction};
//     use pollster::block_on;
//     use crate::gpu::combined_network::{NetworkWithSensorValues, CombinedNetwork};

//     use super::Gpu;

//     #[test]
//     fn run_simple_networks() {
//         let gpu_opt = block_on(Gpu::new("".to_string()));
//         let mut gpu = gpu_opt.unwrap();
//         gpu.setup_compute_layer_shader_module();

//         let configuration  = Configuration::neat(
//             Box::new(vec![
//                 NodeGene::new(1, NodeKind::Sensor),
//                 NodeGene::new_hidden(2, ActivationFunction::SIGMOID),
//                 NodeGene::new(3, NodeKind::Output),
//                 NodeGene::new(4, NodeKind::Output),
//                 NodeGene::new(5, NodeKind::Output),
//             ]), 0.0);
        
//             let mut run_context =  RunContext::new(4, 0);
//             let mut network_1 = NeatGenome::minimal(&configuration, &mut run_context);
//             network_1.genes.connect.clear();
//             network_1.genes.connect.add(ConnectGene::new_with_weight(1, 2, 1.0 ,true));
//             network_1.genes.connect.add(ConnectGene::new_with_weight(2, 3, 1.0, true));
//             network_1.genes.connect.add(ConnectGene::new_with_weight(2, 4, 1.0, true));
//             let network_2 = network_1.clone();
//             let network_3 = network_1.clone();

//             let mut combined_network = CombinedNetwork::from_networks(vec![
//                 NetworkWithSensorValues{
//                     network: network_1,
//                     sensor_values: vec![vec![0.1]]
//                 },                
//                 NetworkWithSensorValues{
//                     network: network_2,
//                     sensor_values: vec![vec![0.1]]
//                 },
//                 NetworkWithSensorValues{
//                     network: network_3,
//                     sensor_values: vec![vec![0.1]]
//                 }
//             ]);

//             pollster::block_on(gpu.compute_combined_network(&mut combined_network));
    
//             println!("{:?}", combined_network);
//     }

// }