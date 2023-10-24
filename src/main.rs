mod camera;
mod map;
mod player;

use bevy::prelude::*;
use camera::plugin::CameraPlugin;
use map::plugin::MapPlugin;
use player::plugin::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((CameraPlugin, DefaultPlugins, MapPlugin, PlayerPlugin))
        .run();
}
