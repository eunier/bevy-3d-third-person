mod camera;
mod world;

use bevy::prelude::*;
use camera::plugin::CameraPlugin;
use world::plugin::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((CameraPlugin, WorldPlugin, DefaultPlugins))
        .run();
}
