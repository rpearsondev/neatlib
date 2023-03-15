use bevy::prelude::*;

pub struct RotatingSphere;
#[derive(Component)]
struct Sphere;


impl Plugin for RotatingSphere {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(add_rotating_sphere)
        .add_system(rotate_sphere);
    }
}

fn add_rotating_sphere(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    }).insert(Sphere{});
    
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn rotate_sphere(time: Res<Time>,mut q: Query<&mut Transform, With<Sphere>> ){
    for mut transform in q.iter_mut() {
        transform.rotate(Quat::from_rotation_z(0.3 * time.delta_seconds()));
    }
}
