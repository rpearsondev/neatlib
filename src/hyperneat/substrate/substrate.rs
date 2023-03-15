
use serde::{Serialize, Deserialize};

use super::{substrate_type::SubstrateType, substrate_coordinate_scheme::SubstrateCoordinateScheme};

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Substrate{
    pub substrate_type: SubstrateType,
    pub number_of_nodes: u32,
    pub connected_to_layers: Vec<usize>,
    pub coordinate_scheme: SubstrateCoordinateScheme
}
impl Substrate{
    pub fn new(
        substrate_type: SubstrateType, 
        number_of_nodes: u32) -> Self{
        Self{
            substrate_type,
            number_of_nodes,
            connected_to_layers: vec![],
            coordinate_scheme: SubstrateCoordinateScheme::CenterOut
        }
    }
    pub fn get_node_positions(&self) -> Vec<Vec<i32>>{
        if self.number_of_nodes == 1{
            return vec![vec![0,0]];
        }

        let number_of_nodes = self.number_of_nodes;
        let mut root = 1;
        for n in (1..1000).step_by(2){
            let size = n*n;
            if size >= number_of_nodes{
                root = n;
                break;
            }
        }
      
        let mut results: Vec<Vec<i32>> = Vec::new();
        let nodes_from_center = ((root-1) / 2) as i32;
        for x in -nodes_from_center..=nodes_from_center{
            for y in -nodes_from_center..=nodes_from_center{
                results.push(vec![x, y])
            }
        }
        
        let mut left = true;
        while results.len() > number_of_nodes as usize{
            if left{
                results.remove(0);
            }else{
                results.remove(results.len()-1);
            }
            left = !left;
        }
        
        results
    }
}

#[cfg(test)]
mod substrate_tests{
    use crate::hyperneat::substrate::substrate_type::SubstrateType;
    use super::Substrate;

    #[test]
    fn get_node_positions_1(){
        let substrate = Substrate::new(SubstrateType::Hidden, 1);
        let node_positions = substrate.get_node_positions();
        assert_eq!(format!("{:?}", node_positions), "[[0, 0]]");
    }

    #[test]
    fn get_node_positions_2(){
        let substrate = Substrate::new(SubstrateType::Hidden, 2);
        let node_positions = substrate.get_node_positions();
        assert_eq!(format!("{:?}", node_positions), "[[0, 0], [0, 1]]");
    }

    #[test]
    fn get_node_positions_3(){
        let substrate = Substrate::new(SubstrateType::Hidden, 3);
        let node_positions = substrate.get_node_positions();
        assert_eq!(format!("{:?}", node_positions), "[[0, -1], [0, 0], [0, 1]]");
    }
    
    #[test]
    fn get_node_positions_9(){
        let substrate = Substrate::new(SubstrateType::Input, 9);
        let node_positions = substrate.get_node_positions();
        println!("{:?}", node_positions);
        assert_eq!(node_positions.len(), 9);
    }
}