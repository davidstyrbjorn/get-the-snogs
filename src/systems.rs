use std::{borrow::Borrow, f32::consts::PI};

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, utils::HashSet};
use bevy_rapier3d::prelude::{Collider, LockedAxes, RigidBody, Velocity};

use crate::{components::*, SnogSpawnerTimer};

use rand::prelude::*;

// Setup a simple 2D scene
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(30.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        Collider::cuboid(15.0, 0.01, 15.0),
    ));

    // moveable cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                depth: 0.5,
                radius: 0.25,
                ..default()
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Collider::cylinder(0.5, 0.25),
        RigidBody::Dynamic,
        Velocity::zero(),
        LockedAxes::ROTATION_LOCKED,
        Player {},
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
        projection: Projection::Perspective(PerspectiveProjection {
            fov: PI / 3.0,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 6.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.5, 0.5, 1.0)),
            ..default()
        },
        ..default()
    });
}

pub fn spawn_trees(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut batch_bag = vec![];
    let mut rng = rand::thread_rng();

    for _ in 0..80 {
        let r = rng.gen_range(4.0..15.0) as f32;
        let random_angle = rng.gen_range(0.0..(2.0 * PI));
        let x = r * f32::cos(random_angle);
        let y = r * f32::sin(random_angle);

        let mut scene = asset_server.load("models/tree_tall.glb#Scene0");
        if rng.gen_range(0..10) > 8 {
            scene = asset_server.load("models/tree_plateau.glb#Scene0");
        } else if rng.gen_range(0..10) > 8 {
            scene = asset_server.load("models/tree_thin.glb#Scene0");
        }

        // Make entity!
        batch_bag.push((
            SceneBundle {
                scene: scene,
                transform: Transform::from_xyz(x, 0.0, y).with_scale(Vec3::new(1.5, 1.5, 1.5)),
                ..default()
            },
            Collider::cylinder(5.0, 0.2),
        ));
    }
    commands.spawn_batch(batch_bag);
}

pub fn rotate_camera_system(time: Res<Time>, mut query: Query<&mut Transform, With<Camera3d>>) {
    let t = time.elapsed_seconds() * 0.4;
    let x = t.sin() * 10.0;
    let z = t.cos() * 10.0;
    let y: f32 = 3.0;

    let mut transform = query.single_mut();
    transform.translation = Vec3::new(x, y, z);
    transform.look_at(Vec3::ZERO, Vec3::Y);
}

const MOVE_SPEED: f32 = 3.0;

pub fn player_move_system(
    mut query: Query<&mut Velocity, With<Player>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut velocity = query.single_mut();
    if keys.pressed(KeyCode::Left) {
        velocity.linvel.x = -MOVE_SPEED;
    } else if keys.pressed(KeyCode::Right) {
        velocity.linvel.x = MOVE_SPEED;
    } else {
        velocity.linvel.x *= 0.5 * time.delta_seconds();
    }
    if keys.pressed(KeyCode::Up) {
        velocity.linvel.z = -MOVE_SPEED;
    } else if keys.pressed(KeyCode::Down) {
        velocity.linvel.z = MOVE_SPEED;
    } else {
        velocity.linvel.z *= 0.5 * time.delta_seconds();
    }
}

pub fn snog_spawner_system(
    mut commands: Commands,
    timer: Res<Time>,
    mut snog_timer: ResMut<SnogSpawnerTimer>,
) {
    snog_timer.timer.tick(timer.delta());
    if snog_timer.timer.finished() {
        println!("Snog timer finished");
        commands.spawn();
    }
}
