use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_asset_loader::prelude::*;

use std::{f32::consts::TAU, ops::Mul};

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

fn player_movement_system(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>
) {
    let player_move_speed = 1.;
    let player_rotation_speed = 0.25;
    for (mut player_tform, mut player_velocity) in player_query.iter_mut() {
        player_velocity.linvel = if keys.pressed(KeyCode::Up) {
            Vec3::new(0., 0., -player_move_speed)
        } else if keys.pressed(KeyCode::Down) {
            Vec3::new(0., 0., player_move_speed)
        } else {
            Vec3::new(0., -9.81, 0.)
        };

        player_velocity.angvel = if keys.pressed(KeyCode::Left) {
            Vec3::new(0., player_rotation_speed, 0.)
        } else if keys.pressed(KeyCode::Right) {
            Vec3::new(0., -player_rotation_speed, 0.)
        } else {
            Vec3::ZERO
        };

        //this is a kludge because the mesh doesn't face the correct direction
        let true_forward = player_tform.forward();
        let forward_direction = Vec3::new(true_forward.z, 0., -true_forward.x);
        player_tform.translation += forward_direction * player_velocity.linvel * time.delta_seconds();
        player_tform.rotate_y(player_velocity.angvel.mul(TAU * time.delta_seconds()).y);
    }   
}

fn spawn_player(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    my_mesh_assets: Res<MyMeshAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //create collider from mesh
    let rover_mesh = meshes.get(&my_mesh_assets.rover).unwrap();
    let collider = Collider::from_bevy_mesh(rover_mesh, &ComputedColliderShape::TriMesh);

    let player = (
        PbrBundle {
            mesh: my_mesh_assets.rover.clone(),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_scale(Vec3::new(0.25, 0.25, 0.25))
                                    .with_translation(Vec3::new(0., 1.5, 0.)),
            ..default()
        },
        Player,
        Velocity { 
            linvel: Vec3::ZERO,
            angvel: Vec3::ZERO,
         },
         RigidBody::Dynamic,
        collider.unwrap(),
    );

    let camera = Camera3dBundle {
        transform: Transform::from_xyz(-12.1, 4.6, 0.)
                                .with_rotation(Quat::from_euler(EulerRot::ZYX, -0.1, 4.7, 0.)),
        ..default()
    };

    let player_entity = commands.spawn(player).id();
    let camera_entity = commands.spawn(camera).id();

    commands.entity(player_entity).push_children(&[camera_entity]);
}
