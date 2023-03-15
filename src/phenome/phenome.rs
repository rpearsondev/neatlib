use crate::{phenome::{node_index_lookup::NodePositionLookup, phenome_layer::PhenomeLayer}, common::{network_definition_node_layer_resolver::NetworkDefinitionNodeLayerResolver, network_definition::{NetworkDefinition, NetworkDefinitionNode}, NeatFloat}, node_kind::NodeKind};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct Phenome{
    pub id: Uuid,
    pub layers: Vec<PhenomeLayer>,
    expected_results: usize,
    outputs_locations: Vec<usize>,
    sensor_locations:Vec<usize>
}
impl Phenome {
    pub fn activate(&self, sensor_values: &Vec<NeatFloat>) -> Vec<NeatFloat>{
        let mut state = vec![0.0; self.expected_results];
        self.activate_internal(sensor_values, &mut state);
        self.extract_outputs(&state)
    }
    pub fn activate_recurrent(&self, inputs: &Vec<Vec<NeatFloat>>) -> Vec<NeatFloat>{
        let mut state = vec![0.0; self.expected_results];
        for item in inputs{
            self.activate_internal(item, &mut state);
        }
        self.extract_outputs(&state)
    }
    pub fn activate_recurrent_external_state(&self, inputs: &Vec<NeatFloat>, mut state: &mut Vec<NeatFloat>) -> Vec<NeatFloat>{
        if state.len() == 0{
            state.extend(vec![0.0; self.expected_results]);
        }
        self.activate_internal(inputs, &mut state);
        self.extract_outputs(&state)
    }
    fn extract_outputs(&self, state: &Vec<NeatFloat>) -> Vec<NeatFloat>{
        let mut results = vec![0.0; self.outputs_locations.len()];
        let mut c = 0 as usize;
        for n in &self.outputs_locations{
            results[c] = state[*n];
            c +=1;
        }
        results
    }
    fn activate_internal(&self, inputs: &Vec<NeatFloat>, mut state:&mut Vec<NeatFloat>){
        let number_of_layers = self.layers.len();

        if number_of_layers > 1 {
            self.layers[0].activate_sensors(inputs, &mut state, &self.sensor_locations);
            for i in 1..number_of_layers {
                self.layers[i].activate_layer(&mut state);
            }
        }
    }

    pub fn from_network_schema<TSchema>(schema: &TSchema) -> Self where TSchema:NetworkDefinition{
        
        let node_index_lookup = NodePositionLookup::new(schema);
        let resolved_node_layers = NetworkDefinitionNodeLayerResolver::get_node_layers(schema, false);
        
        let mut layers:Vec<PhenomeLayer> = Vec::with_capacity(resolved_node_layers.layers.len());

        let mut layer_number: u32 = 0;
        while layer_number < resolved_node_layers.layers.len() as u32{
            let layer_nodes = &resolved_node_layers.layers[layer_number as usize];
            if layer_nodes.len() == 0{
                break;
            }
            let layer = PhenomeLayer::new(layer_nodes, schema, &node_index_lookup);
            layers.push(layer);
            layer_number +=1;
        }

        let all_nodes = schema.get_all_nodes();
        let sensors = all_nodes.iter().filter(|n| n.kind == NodeKind::Sensor).collect::<Vec<&NetworkDefinitionNode>>();
        let outputs = all_nodes.iter().filter(|n| n.kind == NodeKind::Output).collect::<Vec<&NetworkDefinitionNode>>();

        let mut sensor_locations : Vec<usize> = Vec::with_capacity(sensors.len());
        let mut output_locations : Vec<usize> = Vec::with_capacity(outputs.len());
        
        for s in sensors{
            sensor_locations.push(node_index_lookup.get_node_position(s.identity));
        }
        
        for o in outputs{
            output_locations.push(node_index_lookup.get_node_position(o.identity));
        }

        Phenome {
            id: schema.get_network_identifier(),
            layers: layers,
            expected_results: node_index_lookup.len() as usize,
            outputs_locations: output_locations,
            sensor_locations: sensor_locations
        }
    }
    pub fn empty() -> Self{
        Phenome {
            id: Uuid::new_v4(),
            layers: vec![],
            expected_results: 0,
            outputs_locations: vec![],
            sensor_locations: vec![]
        }
    }
}
