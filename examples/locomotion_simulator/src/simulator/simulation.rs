use rapier3d::{prelude::*};
use super::{rapier_context::RapierContext, object_definition::ObjectDefinition};

pub struct Simulation{
    pub gravity: bool,
    pub rapier_context: RapierContext
}

impl Default for Simulation{
    fn default() -> Self {
        Self { gravity: true, rapier_context: RapierContext::default() }
    }
}

impl Simulation{  
    pub fn gravity(&mut self, use_gravity: bool ) {
        self.gravity = use_gravity;
    }

    pub fn add_object<T>(&mut self, mut object: T) -> T where T: ObjectDefinition{
        object.create_in_engine(&mut self.rapier_context);
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
