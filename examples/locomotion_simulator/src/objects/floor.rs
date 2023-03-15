use bevy::{prelude::{PbrBundle, Mesh, Commands, ResMut, StandardMaterial, Assets, Color, Transform}, render::view::RenderLayers};
use rapier3d::prelude::*;
use crate::simulator::{rapier_context::{RapierContext}, object_definition::ObjectDefinition};
use bevy::render::mesh::shape::Box;

const TILE_WIDTH : f32 = 20.0;
const HALF_TILE_WIDTH : f32 = TILE_WIDTH / 2.0;
const GAP: f32 = 0.5;
const HALF_GAP: f32 = GAP / 2.0;
const TILES_OUTWARDS_X: i32 = 1;
const TILES_OUTWARDS_MINUS_X: i32 = -1;
const TILES_OUTWARDS_Z: i32 = 1;
const TILES_OUTWARDS__MINUS_Z: i32 = -5;

#[derive(Clone)]
pub struct Floor;

impl Floor {
    pub fn new() -> Self{
        Self
    }
}

impl Floor {
    pub fn bevy_setup(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, standard_materials: &mut ResMut<Assets<StandardMaterial>>, render_layer: u8){
        for x in TILES_OUTWARDS_MINUS_X..TILES_OUTWARDS_X{
            for z in TILES_OUTWARDS__MINUS_Z..TILES_OUTWARDS_Z{
                commands.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(Box{min_x: -(HALF_TILE_WIDTH - HALF_GAP), min_y: -1.0, min_z: -(HALF_TILE_WIDTH - HALF_GAP), max_x:(HALF_TILE_WIDTH - HALF_GAP), max_y: if x % 2 == 0 {0.2} else {0.0}, max_z: (HALF_TILE_WIDTH - HALF_GAP)})),
                    material:  standard_materials.add(Color::PURPLE.into()),
                    transform: Transform::from_xyz(x as f32 * TILE_WIDTH, 0.0, z as f32 * TILE_WIDTH),
                    ..Default::default()
                }).insert(RenderLayers::layer(render_layer));
            }
        }
    }
}

impl ObjectDefinition for Floor{
    fn create_in_engine(&mut self, context: &mut RapierContext) {
        for x in TILES_OUTWARDS_MINUS_X..TILES_OUTWARDS_X{
            for z in TILES_OUTWARDS__MINUS_Z..TILES_OUTWARDS_Z{
                let collider = ColliderBuilder::cuboid(TILE_WIDTH - GAP, if x % 2 == 0 {0.2} else {0.0}, TILE_WIDTH - GAP)
                .collision_groups(InteractionGroups::new(Group::GROUP_2, Group::ALL))
                .translation(vector![x as f32 * TILE_WIDTH, 0.0, z as f32 * TILE_WIDTH])
                .build();
                context.collider_set.insert(collider); 
            }
        }
              
    }
}