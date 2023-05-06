use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

mod components;
mod systems;

use crate::systems::*;

#[derive(Resource)]
pub struct SnogSpawnerTimer {
    timer: Timer,
}

fn main() {
    App::new()
        .add_plugins(bevy::DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(SnogSpawnerTimer {
            timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        })
        .add_startup_system(setup)
        .add_startup_system(spawn_trees)
        .add_system(player_move_system)
        .add_system(snog_spawner_system)
        // .add_system(rotate_camera_system)
        .run();
}
