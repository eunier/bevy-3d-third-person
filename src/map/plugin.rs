use bevy::prelude::*;

use super::system::{spawn_floor, spawn_light};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_floor, spawn_light));
    }
}
