use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_asset_loader::prelude::*;

pub const ROVER_SIZE: Vec3 = Vec3::new(0.5, 0.2, 1.05);

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


#[derive(Component)]
pub struct Wheel;

#[derive(AssetCollection, Resource)]
pub struct MyMeshAssets {
    #[asset(path = "rovie.glb#Mesh0/Primitive0")]
    rover: Handle<Mesh>,
}

#[derive(Component)]
pub struct Player;

pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut wheels: Query<(&mut ExternalForce, &Transform, With<Wheel>)>,
) {
    // TORQUE
    let torque: f32 = 5.;
    if keyboard_input.pressed(KeyCode::Up) {
        for (mut forces, transform, _) in wheels.iter_mut() {
            forces.torque = transform.rotation * Vec3::new(0., -torque, 0.);
        }
    }
    if keyboard_input.pressed(KeyCode::Down) {
        for (mut forces, transform, _) in wheels.iter_mut() {
            forces.torque = transform.rotation * Vec3::new(0., torque, 0.);
        }
    }
    if keyboard_input.just_released(KeyCode::Up) {
        for (mut forces, transform, _) in wheels.iter_mut() {
            forces.torque = Vec3::ZERO;
        }
    }
    if keyboard_input.just_released(KeyCode::Down) {
        for (mut forces, transform, _) in wheels.iter_mut() {
            forces.torque = Vec3::ZERO;
        }
    }
}

fn spawn_player(
    mut commands: Commands,
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

    let rot_angle = 90.0_f32.to_radians();

    let front_left_tform = Vec3::new(-ROVER_SIZE.x / 2., -0.1, -ROVER_SIZE.z / 2.);
    let front_right_tform = Vec3::new(ROVER_SIZE.x / 2., -0.1, -ROVER_SIZE.z / 2.);
    let rear_left_tform = Vec3::new(-ROVER_SIZE.x / 2., -0.1, ROVER_SIZE.z / 2.);
    let rear_right_tform = Vec3::new(ROVER_SIZE.x / 2., -0.1, ROVER_SIZE.z / 2.);
    
    //todo: DRY!!!
    let left_front_wheel = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(cylinder)),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_translation(front_left_tform).with_rotation(Quat::from_rotation_z(rot_angle)),
            ..default()
        },
        Wheel,
        RigidBody::Dynamic,
        Collider::cylinder(cylinder.height / 2., cylinder.radius),
        ExternalForce::default(),
        Friction::coefficient(50.),
        Restitution::coefficient(0.7),
        AdditionalMassProperties::Mass(10.),
    );

    let right_front_wheel = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(cylinder)),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_translation(front_right_tform).with_rotation(Quat::from_rotation_z(rot_angle)),
            ..default()
        },
        Wheel,
        RigidBody::Dynamic,
        Collider::cylinder(cylinder.height / 2., cylinder.radius),
        ExternalForce::default(),
        Friction::coefficient(50.),
        Restitution::coefficient(0.7),
        AdditionalMassProperties::Mass(10.),
    );

    let left_rear_wheel = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(cylinder)),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_translation(rear_left_tform).with_rotation(Quat::from_rotation_z(rot_angle)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cylinder(cylinder.height / 2., cylinder.radius),
        ExternalForce::default(),
        Friction::coefficient(50.),
        Restitution::coefficient(0.7),
        AdditionalMassProperties::Mass(10.),
    );

    let right_rear_wheel = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(cylinder)),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_translation(rear_right_tform).with_rotation(Quat::from_rotation_z(rot_angle)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cylinder(cylinder.height / 2., cylinder.radius),
        ExternalForce::default(),
        Friction::coefficient(50.),
        Restitution::coefficient(0.7),
        AdditionalMassProperties::Mass(10.),
    );

    //spawn body
    // let rover_collider_mesh = meshes.get(&my_mesh_assets.rover).unwrap();
    // let collider = Collider::from_bevy_mesh(rover_collider_mesh, &ComputedColliderShape::TriMesh).unwrap();
    let body = (
        PbrBundle {
            // mesh: my_mesh_assets.rover.clone(),
            mesh: meshes.add(Mesh::from(shape::Cube::new(0.25))),
            material: materials.add(Color::BEIGE.into()),
            transform: Transform::from_scale(Vec3::new(0.5, 0.5, 0.5))
                                    .with_translation(Vec3::new(0., 1.5, 0.)),
            ..default()
        },
        Player,
        RigidBody::Dynamic,
        // collider,
        Collider::cuboid(0.2, 0.1, 0.2),
        ColliderMassProperties::Density(10.),
    );

    let camera = Camera3dBundle {
        transform: Transform::from_xyz(0., 2., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    let body_entity = commands.spawn(body).with_children(|children| {
        children.spawn(camera);
    }).id();

    let fl_axle = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(0.1))),
            material: materials.add(Color::GRAY.into()),
            transform: Transform::from_translation(front_left_tform),
            ..default()
        },
        RigidBody::Dynamic,
    );

    let fr_axle = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(0.1))),
            material: materials.add(Color::GRAY.into()),
            transform: Transform::from_translation(front_right_tform),
            ..default()
        },
        RigidBody::Dynamic,
    );

    let rl_axle = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(0.1))),
            material: materials.add(Color::GRAY.into()),
            transform: Transform::from_translation(rear_left_tform),
            ..default()
        },
        RigidBody::Dynamic,
    );

    let rr_axle = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(0.1))),
            material: materials.add(Color::GRAY.into()),
            transform: Transform::from_translation(rear_right_tform),
            ..default()
        },
        RigidBody::Dynamic,
    );

    let fl_axle_entity = commands.spawn(fl_axle).id();
    let fr_axle_entity = commands.spawn(fr_axle).id();
    let rl_axle_entity = commands.spawn(rl_axle).id();
    let rr_axle_entity = commands.spawn(rr_axle).id();

    let stiffness = 5.;
    let damping = 0.2;
    let fl_suspension_joint = PrismaticJointBuilder::new(Vec3::Y)
        .local_anchor1(front_left_tform)
        .local_anchor2(Vec3::new(front_left_tform.x, -0.1, front_left_tform.z))
        .motor_position(0., stiffness, damping)
        .limits([-0.2, 0.2]);
    let fr_suspension_joint = PrismaticJointBuilder::new(Vec3::Y)
        .local_anchor1(front_right_tform)
        .local_anchor2(Vec3::new(front_right_tform.x, -0.1, front_right_tform.z))
        .motor_position(0., stiffness, damping)
        .limits([-0.2, 0.2]);
    let rl_suspension_joint = PrismaticJointBuilder::new(Vec3::Y)
        .local_anchor1(rear_left_tform)
        .local_anchor2(Vec3::new(rear_left_tform.x, -0.1, rear_left_tform.z))
        .motor_position(0., stiffness, damping)
        .limits([-0.2, 0.2]);
    let rr_suspension_joint = PrismaticJointBuilder::new(Vec3::Y)
        .local_anchor1(rear_right_tform)
        .local_anchor2(Vec3::new(rear_right_tform.x, -0.1, rear_right_tform.z))
        .motor_position(0., stiffness, damping)
        .limits([-0.2, 0.2]);

    commands.entity(fl_axle_entity).insert(MultibodyJoint::new(body_entity, fl_suspension_joint));
    commands.entity(fr_axle_entity).insert(MultibodyJoint::new(body_entity, fr_suspension_joint));
    commands.entity(rl_axle_entity).insert(MultibodyJoint::new(body_entity, rl_suspension_joint));
    commands.entity(rr_axle_entity).insert(MultibodyJoint::new(body_entity, rr_suspension_joint));

    let lf_wheel_entity = commands.spawn(left_front_wheel).id();
    let rf_wheel_entity = commands.spawn(right_front_wheel).id();
    let lr_wheel_entity = commands.spawn(left_rear_wheel).id();
    let rr_wheel_entity = commands.spawn(right_rear_wheel).id();

    let locked_axes = JointAxesMask::X
        | JointAxesMask::Y
        | JointAxesMask::Z
        | JointAxesMask::ANG_Y
        | JointAxesMask::ANG_Z;

    let lf_wheel_joint = GenericJointBuilder::new(locked_axes)
        .local_axis1(Vec3::X)
        .local_axis2(Vec3::Y)
        .local_anchor1(front_left_tform)
        .local_anchor2(Vec3::ZERO)
        .build();
    let rf_wheel_joint = GenericJointBuilder::new(locked_axes)
        .local_axis1(Vec3::X)
        .local_axis2(Vec3::Y)
        .local_anchor1(front_right_tform)
        .local_anchor2(Vec3::ZERO)
        .build();
    let rl_wheel_joint = GenericJointBuilder::new(locked_axes)
        .local_axis1(Vec3::X)
        .local_axis2(Vec3::Y)
        .local_anchor1(rear_left_tform)
        .local_anchor2(Vec3::ZERO)
        .build();
    let rr_wheel_joint = GenericJointBuilder::new(locked_axes)
        .local_axis1(Vec3::X)
        .local_axis2(Vec3::Y)
        .local_anchor1(rear_right_tform)
        .local_anchor2(Vec3::ZERO)
        .build();

    commands.entity(lf_wheel_entity).insert(MultibodyJoint::new(fl_axle_entity, lf_wheel_joint));
    commands.entity(rf_wheel_entity).insert(MultibodyJoint::new(fr_axle_entity, rf_wheel_joint));
    commands.entity(lr_wheel_entity).insert(MultibodyJoint::new(rl_axle_entity, rl_wheel_joint));
    commands.entity(rr_wheel_entity).insert(MultibodyJoint::new(rr_axle_entity, rr_wheel_joint));
}
