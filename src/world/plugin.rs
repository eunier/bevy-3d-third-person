use bevy::prelude::*;

use super::system::{spawn_floor, spawn_ligh};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_floor, spawn_ligh));
    }
}
