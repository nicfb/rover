use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, na::Translation};
use bevy_asset_loader::prelude::*;

pub const ROVER_SIZE: Vec3 = Vec3::new(0.5, 0.2, 1.05);

pub struct PlayerPlugin;

#[derive(Default, Resource)]
pub struct Game {
    fr_wheel_joint: Option<Entity>,
    fl_wheel_joint: Option<Entity>,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::InGame)
                .load_collection::<MyMeshAssets>()
            )
            .init_resource::<Game>()
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
    rover: Handle<Mesh>,
}

#[derive(Component)]
pub struct Player;

fn player_movement_system(
    keys: Res<Input<KeyCode>>,
    game: Res<Game>,
) {
    let mut target_velocity = 0.;
    let factor = 0.7;
    target_velocity = if keys.pressed(KeyCode::Up) {
        10.
    } else if keys.pressed(KeyCode::Down) {
        -10.
    } else {
        0.
    };

    // (game.fl_wheel_joint as ImpulseJoint).data.set_motor_velocity(JointAxis::AngX, target_velocity, factor);
    // (game.fr_wheel_joint as ImpulseJoint).data.set_motor_velocity(JointAxis::AngX, target_velocity, factor);
}

fn spawn_player(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut meshes: ResMut<Assets<Mesh>>,
    my_mesh_assets: Res<MyMeshAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //spawn wheels
    let cylinder = shape::Cylinder {
        height: 0.1,
        radius: 0.1,
        ..default()
    };
    
    let wheel_fl = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(cylinder)),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_translation(Vec3::new(-ROVER_SIZE.x / 2., 0., -ROVER_SIZE.z / 2.)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cylinder(cylinder.height / 2., cylinder.radius)
    );

    let wheel_fr = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(cylinder)),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_translation(Vec3::new(ROVER_SIZE.x / 2., 0., -ROVER_SIZE.z / 2.)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cylinder(cylinder.height / 2., cylinder.radius)
    );

    let wheel_rl = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(cylinder)),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_translation(Vec3::new(-ROVER_SIZE.x / 2., 0., ROVER_SIZE.z / 2.)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cylinder(cylinder.height / 2., cylinder.radius)
    );

    let wheel_rr = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(cylinder)),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_translation(Vec3::new(ROVER_SIZE.x / 2., 0., ROVER_SIZE.z / 2.)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cylinder(cylinder.height / 2., cylinder.radius)
    );

    //spawn body
    // let rover_collider_mesh = meshes.get(&my_mesh_assets.rover).unwrap();
    // let collider = Collider::from_bevy_mesh(rover_collider_mesh, &ComputedColliderShape::TriMesh).unwrap();
    let body = (
        PbrBundle {
            // mesh: my_mesh_assets.rover.clone(),
            mesh: meshes.add(Mesh::from(shape::Cube::new(0.25))),
            material: materials.add(Color::BEIGE.into()),
            // transform: Transform::from_scale(Vec3::new(0.25, 0.25, 0.25))
            //                         .with_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Player,
        RigidBody::Dynamic,
        // collider,
        Collider::cuboid(0.12, 0.12, 0.12)
    );

    let camera = Camera3dBundle {
        transform: Transform::from_xyz(0., 1.5, 3.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    let body_entity = commands.spawn(body).with_children(|children| {
        children.spawn(camera);
    }).id();

    let wheel_fl_entity = commands.spawn(wheel_fl).id();
    let wheel_fr_entity = commands.spawn(wheel_fr).id();
    let wheel_rl_entity = commands.spawn(wheel_rl).id();
    let wheel_rr_entity = commands.spawn(wheel_rr).id();

    let wheel_joint_fl = RevoluteJointBuilder::new(Vec3::Z).local_anchor1(Vec3::new(-ROVER_SIZE.x / 2., 0., -ROVER_SIZE.z / 2.)).local_anchor2(Vec3::ZERO);
    let wheel_joint_fr = RevoluteJointBuilder::new(Vec3::Z).local_anchor1(Vec3::new(ROVER_SIZE.x / 2., 0., -ROVER_SIZE.z / 2.)).local_anchor2(Vec3::ZERO);
    let wheel_joint_rl = RevoluteJointBuilder::new(Vec3::Z).local_anchor1(Vec3::new(-ROVER_SIZE.x / 2., 0., ROVER_SIZE.z / 2.)).local_anchor2(Vec3::ZERO);
    let wheel_joint_rr = RevoluteJointBuilder::new(Vec3::Z).local_anchor1(Vec3::new(ROVER_SIZE.x / 2., 0., ROVER_SIZE.z / 2.)).local_anchor2(Vec3::ZERO);

    // let mut j1 = ImpulseJoint::new(body_entity, wheel_joint_fl);
    // j1.data.set_motor_velocity(JointAxis::AngZ, 10., 0.7); //full throttle for debugging

    // let mut j2 = ImpulseJoint::new(body_entity, wheel_joint_fr);
    // j2.data.set_motor_velocity(JointAxis::AngZ,10., 0.7); //full throttle for debugging

    // game.fl_wheel_joint = Some(commands.entity(wheel_fl_entity).insert(j1).id());
    // game.fr_wheel_joint = Some(commands.entity(wheel_fr_entity).insert(j2).id());
    game.fl_wheel_joint = Some(commands.entity(wheel_fl_entity).insert(ImpulseJoint::new(body_entity, wheel_joint_fl)).id());
    game.fr_wheel_joint = Some(commands.entity(wheel_fr_entity).insert(ImpulseJoint::new(body_entity, wheel_joint_fr)).id());
    commands.entity(wheel_rl_entity).insert(ImpulseJoint::new(body_entity, wheel_joint_rl));
    commands.entity(wheel_rr_entity).insert(ImpulseJoint::new(body_entity, wheel_joint_rr));
}
