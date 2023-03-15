use crate::{neat::genome::neat::node_gene::NodeGene, node_kind::NodeKind};

pub struct NodeConf;

impl NodeConf{
    pub fn simple(sensors:u32, outputs: u32) -> Box<Vec<NodeGene>>{
        let mut result: Vec<NodeGene> = Vec::new();
        
        for i in 1..(sensors+outputs+1){
            if i <= sensors {
                result.push(NodeGene::new(i as i32, NodeKind::Sensor));
            }else{
                result.push(NodeGene::new(i as i32, NodeKind::Output))
            }
        }
        Box::new(result)
    }
}