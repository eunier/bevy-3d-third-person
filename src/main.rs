mod world;

use bevy::prelude::*;
use world::system::spawn_floor;

fn main() {
    spawn_floor();
    App::new().add_plugins(DefaultPlugins).run();
}
