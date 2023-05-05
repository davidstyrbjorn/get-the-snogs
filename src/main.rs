use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::{Collider, NoUserData, RapierPhysicsPlugin, RigidBody},
    render::RapierDebugRenderPlugin,
};

// For graphics
// https://www.kenney.nl/assets/nature-kit

fn main() {
    App::new()
        .add_plugins(bevy::DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(rotate_camera_system)
        .run();
}

struct 

fn rotate_camera_system(time: Res<Time>, mut query: Query<&mut Transform, With<Camera3d>>) {
    let t = time.elapsed_seconds() * 0.4;
    let x = t.sin() * 10.0;
    let z = t.cos() * 10.0;
    let y: f32 = 3.0;

    let mut transform = query.single_mut();
    transform.translation = Vec3::new(x, y, z);
    transform.look_at(Vec3::ZERO, Vec3::Y);
}

// Setup a simple 2D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(15.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        Collider::cuboid(15.0, 0.01, 15.0),
    ));

    // moveable cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Collider::cuboid(0.5, 0.5, 0.5),
        RigidBody::Dynamic,
    ));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 8.0, -2.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-4.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
