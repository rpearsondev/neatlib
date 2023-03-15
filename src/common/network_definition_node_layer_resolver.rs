use hashbrown::HashMap;

use super::network_definition::{NetworkDefinitionNode, NetworkDefinition};

pub struct NetworkDefinitionNodeLayerResolver;

pub struct ResolvedNodeLayers{
    pub layers: Vec<Vec<NetworkDefinitionNode>>,
    pub total_connections: u32,
    pub total_nodes: u32,
    pub total_outputs: Option<u32>
}

impl NetworkDefinitionNodeLayerResolver{
    pub fn get_node_layers<'a, TSchema>(schema: &'a TSchema, include_output_count: bool) -> ResolvedNodeLayers where TSchema:NetworkDefinition{
        
        let schema_connect_genes = schema.get_all_connections();
        let mut nodes : HashMap<i32, u32> = HashMap::new();
        let mut has_made_change = true;
        while has_made_change {
            has_made_change = false;
            for i in 0..schema_connect_genes.len(){
                let connect_gene = &schema_connect_genes[i];
                if !connect_gene.is_enabled || connect_gene.is_recurrent {
                    continue;
                }
                let mut has_in = nodes.contains_key(&connect_gene.connection_in);
                let mut has_out = nodes.contains_key(&connect_gene.connection_out);
                if !has_in {
                    nodes.insert(connect_gene.connection_in, 0); 
                }
                if !has_out {
                    nodes.insert(connect_gene.connection_out, 1);
                }
                has_in = nodes.contains_key(&connect_gene.connection_in);
                has_out = nodes.contains_key(&connect_gene.connection_out);
                if has_out {
                    if has_in{
                        let current_in = nodes[&connect_gene.connection_in];
                        if nodes[&connect_gene.connection_out] < current_in + 1 {
                            nodes.insert(connect_gene.connection_out, current_in + 1);
                            has_made_change = true;
                        }
                    }
                }
            }
        }
        let nodes_with_layers = nodes.iter().map(|(a,b)| (a.clone(), b.clone())).collect::<Vec<(i32, u32)>>();
        let max_layer = nodes_with_layers.iter().map(|x| x.1).max();
        let mut results: Vec<Vec<NetworkDefinitionNode>> = Vec::new(); 

        let mut output_nodes: Option<u32> = None;
        if include_output_count {
            output_nodes = Some(schema.get_output_nodes_count());
        }

        if max_layer.is_none() {
            return ResolvedNodeLayers{ 
                layers: results,
                total_connections: schema_connect_genes.len() as u32,
                total_nodes: 0,
                total_outputs: output_nodes
             };
        }

        let max_layer_usize = max_layer.unwrap() as usize;
        for _ in 0..=max_layer_usize{
            results.push(Vec::new());
        }

        for i in 0..=max_layer.unwrap() as usize{
            let nodes_id_on_layer: Vec<i32> = nodes_with_layers.iter().filter(|x| x.1 == i as u32).map(|x| x.0).collect();
            for node_id_on_layer in nodes_id_on_layer{
                let node_to_push = schema.get_node(node_id_on_layer);
                results[i].push(node_to_push);
            }
        }

        ResolvedNodeLayers{
            layers:results,
            total_connections: schema_connect_genes.len() as u32,
            total_nodes: schema.get_nodes_len(),
            total_outputs: output_nodes
        }
    }
}