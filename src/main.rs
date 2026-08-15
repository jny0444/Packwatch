use avian3d::prelude::*;
use bevy::{
    DefaultPlugins,
    app::{App, Startup},
};
use packwatch::{camera::CameraPlugin, player::PlayerPlugin, scene::spawn_plane::setup};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            CameraPlugin,
            PlayerPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}
