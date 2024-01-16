use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_asset_loader::prelude::*;

use std::f32::consts::TAU;

pub struct PlayerPlugin;


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::InGame)
                .load_collection::<MyMeshAssets>()
            )
            .add_systems(OnEnter(GameState::InGame), spawn_player)
            .add_systems(Update, player_movement_system);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    InGame,
}

#[derive(AssetCollection, Resource)]
pub struct MyMeshAssets {
    #[asset(path = "rovie.glb#Mesh0/Primitive0")]
    rover: Handle<Mesh>
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity {
	pub value: Vec3
}

fn player_movement_system(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>
) {
    let player_move_speed = 1.;
    let player_rotation_speed = 0.15;
    for (mut player_tform, mut player_velocity) in player_query.iter_mut() {
        player_velocity.value.z = if keys.pressed(KeyCode::Up) {
            player_move_speed
        } else if keys.pressed(KeyCode::Down) {
            -player_move_speed
        } else {
            0.
        };

        player_velocity.value.y = if keys.pressed(KeyCode::Left) {
            player_rotation_speed
        } else if keys.pressed(KeyCode::Right) {
            -player_rotation_speed
        } else {
            0.
        };

        let forward_direction = player_tform.forward();
        player_tform.translation += forward_direction * (player_velocity.value.z * time.delta_seconds());
        player_tform.rotate_y(player_velocity.value.y * TAU * time.delta_seconds());
    }   
}

fn spawn_player(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    my_mesh_assets: Res<MyMeshAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let rover_mesh = meshes.get(&my_mesh_assets.rover).unwrap();
    let collider = Collider::from_bevy_mesh(rover_mesh, &ComputedColliderShape::TriMesh);
    let player = (
        PbrBundle {
            mesh: my_mesh_assets.rover.clone(),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_scale(Vec3::new(0.25, 0.25, 0.25))
                                    .with_translation(Vec3::new(0., 0.5, 0.)),
            ..default()
        },
        Player,
        Velocity { value: Vec3::ZERO },
        RigidBody::Dynamic,
        collider.unwrap(),
    );

    let camera = Camera3dBundle {
        transform: Transform::from_xyz(0., 5., 15.)
                                .with_rotation(Quat::from_euler(EulerRot::XYZ, -0.26, 0., 0.)),
        ..default()
    };

    let player_entity = commands.spawn(player).id();
    let camera_entity = commands.spawn(camera).id();

    commands.entity(player_entity).push_children(&[camera_entity]);
}
