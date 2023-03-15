use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;
use uuid::Uuid;
use crate::neat::genome::neat::NeatGenome;
use super::GenerationMember;

pub struct MembersLookup{
    lookup: HashMap<u64, usize, nohash_hasher::BuildNoHashHasher<u64>>
}
impl MembersLookup {
    pub fn new(members: &Vec<GenerationMember<NeatGenome>>) -> Self {
        
        let mut lookup:  HashMap<u64, usize, nohash_hasher::BuildNoHashHasher<u64>> = HashMap::with_capacity_and_hasher(members.len(), BuildNoHashHasher::default());
        let mut c = 0 as usize;
        for member in members{
            lookup.insert(member.genome.id.as_u64_pair().0, c);
            c+= 1;
        }
        MembersLookup{
            lookup: lookup
        }
    }
    pub fn get_array_index(&self, id: Uuid) -> usize{
        *self.lookup.get_key_value(&id.as_u64_pair().0).unwrap().1
    }
    pub fn len(&self) -> usize{
        self.lookup.len()
    }
}
