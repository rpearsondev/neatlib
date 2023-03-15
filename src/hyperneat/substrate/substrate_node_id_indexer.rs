use hashbrown::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default)]
#[derive(Serialize, Deserialize)]
pub struct SubstrateNodeIdIndexer{
    node_hashset: HashMap<(i32, i32, i32), i32>,
    latest_number: i32
}

impl SubstrateNodeIdIndexer{
    pub fn new() -> Self{
        Self{
            node_hashset: HashMap::new(),
            latest_number: 1
        }
    }
    pub fn get_id(&mut self, node_position: (i32, i32, i32)) -> i32 {
        
        let id = self.node_hashset.insert(node_position, self.latest_number);
        if id.is_some(){
            return id.unwrap();
        }
        self.latest_number += 1;
        return self.latest_number - 1;
    }
}