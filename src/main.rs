use avian3d::prelude::*;
use bevy::{DefaultPlugins, app::App};
use packwatch::PackwatchPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PackwatchPlugin,
        ))
        .run();
}
