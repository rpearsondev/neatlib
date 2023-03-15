use crate::simulator::rapier_context::RapierContext;
use super::object_engine_handle::ObjectEngineHandle;

pub trait ObjectDefinition{
    fn create_in_engine(&mut self, context: &mut RapierContext);
    fn get_engine_handle(&self) -> &Option<ObjectEngineHandle>;
}
