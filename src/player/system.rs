use bevy::prelude::*;

use super::component::{Player, Speed};

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Speed(2.0),
        Player,
    );

    commands.spawn(player);
}

pub fn move_player(
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut player_transform, player_speed) in player_query.iter_mut() {
        let camera_transform = match camera_query.get_single() {
            Ok(c) => c,
            Err(err) => Err(format!("Error retriving camera: {}", err)).unwrap(),
        };

        let mut direction = Vec3::ZERO;

        if input.pressed(KeyCode::W) {
            direction += camera_transform.forward();
        }

        if input.pressed(KeyCode::S) {
            direction += camera_transform.back();
        }

        if input.pressed(KeyCode::A) {
            direction += camera_transform.left();
        }

        if input.pressed(KeyCode::D) {
            direction += camera_transform.right();
        }

        direction.y = 0.0;

        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();

        player_transform.translation += movement;
    }
}
