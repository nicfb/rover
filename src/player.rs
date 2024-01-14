use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement_system);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct Speed {
    value: f32
}

fn player_movement_system(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>
) {
    for (mut player_tform, player_speed) in player_query.iter_mut() {
        let cam = match camera_query.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };

        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::Up) {
            direction += cam.forward();
        } else if keys.pressed(KeyCode::Down) {
            direction += cam.back();
        } else if keys.pressed(KeyCode::Left) {
            direction += cam.left();
        } else if keys.pressed(KeyCode::Right) {
            direction += cam.right();
        }

        direction.y = 0.; //fake gravity

        let movement = direction.normalize_or_zero() * player_speed.value * time.delta_seconds();
        player_tform.translation += movement;

        if direction.length_squared() > 0. {
            player_tform.look_to(direction, Vec3::Y);
        }
    }   
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(1., 0.25, 2.))),
        material: materials.add(Color::GRAY.into()),
        transform: Transform::from_xyz(0., 0.5, 0.),
        ..default()
        },
        Player,
        Speed { value: 2.5 }
    );

    commands.spawn(player);
}