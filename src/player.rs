use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_asset_loader::prelude::*;

pub const PLAYER_VELO_Z: f32 = 1.;
pub const PLAYER_ROT_VELO_Y: f32 = 0.25;

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
            .add_systems(Update, (player_movement_system, read_result_system));
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
    mut player_query: Query<(&mut Transform, &mut KinematicCharacterController), With<Player>>
) {
    for (mut player_tform, mut character_controller) in player_query.iter_mut() {
        let mut next_pos = Vec3::ZERO;
        if keys.pressed(KeyCode::Up) {
            next_pos.x += -PLAYER_VELO_Z * time.delta_seconds()
        } else if keys.pressed(KeyCode::Down) {
            next_pos.x += PLAYER_VELO_Z * time.delta_seconds()
        };

        let rot_velocity = if keys.pressed(KeyCode::Left) {
            PLAYER_ROT_VELO_Y
        } else if keys.pressed(KeyCode::Right) {
            -PLAYER_ROT_VELO_Y
        } else {
            0.
        };

        let true_forward = player_tform.forward();
        let forward_direction = Vec3::new(true_forward.z, 0., -true_forward.x);
        let mut tform = forward_direction * next_pos.x;
        player_tform.rotate_y(rot_velocity * TAU * time.delta_seconds());

        tform.y = -1.; //?
        character_controller.translation = Some(tform);
    }   
}

fn read_result_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
    for (entity, output) in controllers.iter() {
        println!("Entity {:?} moved by {:?} and touches the ground: {:?}",
                  entity, output.effective_translation, output.grounded);
    }
}

fn spawn_player(
    mut commands: Commands,
    my_mesh_assets: Res<MyMeshAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (
        PbrBundle {
            mesh: my_mesh_assets.rover.clone(),
            material: materials.add(Color::BEIGE.into()),
            transform: Transform::from_scale(Vec3::new(0.25, 0.25, 0.25))
                                    .with_translation(Vec3::new(0., 1., 0.)),
            ..default()
        },
        Player,
        RigidBody::KinematicPositionBased,
        Collider::cuboid(2., 0.7, 1.5),
        KinematicCharacterController::default()
    );

    let camera = Camera3dBundle {
        transform: Transform::from_xyz(-12.1, 4.6, 0.)
                                .with_rotation(Quat::from_euler(EulerRot::ZYX, -0.1, 4.7, 0.)),
        ..default()
    };

    commands.spawn(player).with_children(|children| {
        children.spawn(camera);
    });
}