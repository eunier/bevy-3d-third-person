mod camera;

mod map;

use bevy::prelude::*;
use camera::plugin::CameraPlugin;
use map::plugin::MapPlugin;

fn main() {
    App::new()
        .add_plugins((CameraPlugin, DefaultPlugins, MapPlugin))
        .run();
}
