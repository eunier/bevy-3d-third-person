mod camera;
mod light;
mod world;

use bevy::prelude::*;
use camera::plugin::CameraPlugin;
use light::plugin::LightPlugin;
use world::plugin::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((CameraPlugin, WorldPlugin, LightPlugin, DefaultPlugins))
        .run();
}
