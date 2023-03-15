use rapier3d::{prelude::*};
use super::{rapier_context::RapierContext, object_definition::ObjectDefinition, object_engine_handle::ObjectEngineHandle};

pub struct Simulation{
    pub gravity: bool,
    pub objects_handles: Vec<ObjectEngineHandle>,
    pub rapier_context: RapierContext
}

impl Default for Simulation{
    fn default() -> Self {
        Self { gravity: true, objects_handles: Vec::new(), rapier_context: RapierContext::default() }
    }
}

impl Simulation{  
    pub fn add_gravity(&mut self) {
        self.gravity = true
    }

    pub fn add_object<T>(&mut self, mut object: T) -> T where T: ObjectDefinition{
        object.create_in_engine(&mut self.rapier_context);
        if object.get_engine_handle().is_some(){
            self.objects_handles.push(object.get_engine_handle().unwrap());
        }
        object
    }
    
    pub fn step(&mut self){
        let mut gravity =vector![0.0, -9.81, 0.0];
        if !self.gravity{
            gravity =vector![0.0, 0.0, 0.0];
        }

        self.rapier_context.physics_pipeline.step(
            &gravity,
            &self.rapier_context.integration_parameters,
            &mut self.rapier_context.island_manager,
            &mut self.rapier_context.broad_phase,
            &mut self.rapier_context.narrow_phase,
            &mut self.rapier_context.rigid_body_set,
            &mut self.rapier_context.collider_set,
            &mut self.rapier_context.impulse_joint_set,
            &mut self.rapier_context.multibody_joint_set,
            &mut self.rapier_context.ccd_solver,
            &self.rapier_context.physics_hooks,
            if self.rapier_context.event_handler.is_some() {self.rapier_context.event_handler.as_ref().unwrap()}  else { &() },
          );
        
    }
}
