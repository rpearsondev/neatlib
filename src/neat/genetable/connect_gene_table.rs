use hashbrown::HashMap;
use nohash_hasher::{BuildNoHashHasher};
use serde::{Deserialize, Serialize};
use crate::neat::genome::neat::connect_gene::ConnectGene;

#[derive(Debug, Clone, Default)]
#[derive(Serialize, Deserialize)]
pub struct ConnectGeneTable{
    connect_gene_hashset: HashMap<u64, u32, nohash_hasher::BuildNoHashHasher<u64>>,
    latest_innovation_number: u32
}

impl ConnectGeneTable{
    pub fn new() -> Self{
        ConnectGeneTable{
            connect_gene_hashset: HashMap::with_hasher(BuildNoHashHasher::default()),
            latest_innovation_number: 1
        }
    }
    pub fn add(&mut self, connect_gene: ConnectGene) -> ConnectGene{
        if self.connect_gene_hashset.insert(connect_gene.connection_hash, self.latest_innovation_number) == None {
            self.latest_innovation_number += 1;
        };
        connect_gene
    }
    pub fn exists(&mut self, connect_gene: &ConnectGene) -> bool{
        self.connect_gene_hashset.contains_key(&connect_gene.connection_hash)
    }
    pub fn len(&self) -> usize{
        self.connect_gene_hashset.len()
    }
    pub fn next_innovation_number(&self) -> u32{
        self.latest_innovation_number
    }
    pub fn reset(&mut self){
        self.connect_gene_hashset.clear();
        self.latest_innovation_number = 1;
    }
}

#[test]
fn same_connections_does_not_increase_len() {
    let mut table = ConnectGeneTable::new();
    table.add(ConnectGene::new(1,1, true));
    let exists = table.exists(&ConnectGene::new(1,1, false));
    let len = table.len();
    let next_innovation_number =table.next_innovation_number();

    assert_eq!(exists, true);
    assert_eq!(len, 1);
    assert_eq!(next_innovation_number, 2);
}

#[test]
fn different_connections_do_increase_len() {
    let mut table = ConnectGeneTable::new();
    table.add(ConnectGene::new(1,1,true));
    table.add(ConnectGene::new(1,2,true));
    let len = table.len();
    let highest_innovation_number =table.next_innovation_number();

    assert_eq!(len, 2);
    assert_eq!(highest_innovation_number, 3);
}

#[test]
fn weight_does_not_increase_len() {
    let mut table = ConnectGeneTable::new();
    table.add(ConnectGene::new(1,1, true));
    table.add(ConnectGene::new(1,1, true));
    let len = table.len();
    let highest_innovation_number =table.next_innovation_number();

    assert_eq!(len, 1);
    assert_eq!(highest_innovation_number, 2);
}