use crate::simulator::rapier_context::RapierContext;

pub trait ObjectDefinition{
    fn create_in_engine(&mut self, context: &mut RapierContext);
}
