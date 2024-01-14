use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light_source, spawn_floor));
    }
}

fn spawn_light_source(mut commands: Commands) {
    let light_source = PointLightBundle {
        point_light: PointLight {
            intensity: 2000.,
            ..default()
        },
        transform: Transform::from_xyz(0., 5., 0.),
        ..default()
    };

    commands.spawn(light_source);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    };

    commands.spawn(floor);
}