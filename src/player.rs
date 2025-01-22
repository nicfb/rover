use bevy::{core::Zeroable, prelude::*};
use bevy_rapier3d::prelude::*;
use bevy_asset_loader::prelude::*;

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
pub struct FrontWheel;

#[derive(AssetCollection, Resource)]
pub struct MyMeshAssets {
    #[asset(path = "rovie.glb#Mesh0/Primitive0")]
    rover: Handle<Mesh>,
}

#[derive(Component)]
pub struct Player;

pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut wheels: Query<(&mut ExternalForce, &mut ImpulseJoint, &Transform, With<FrontWheel>)>,
) {
    let torque: f32 = 60.; //todo: tweak this
    if keyboard_input.pressed(KeyCode::W) {
        for (mut forces, _joint, transform, _) in wheels.iter_mut() {
            forces.torque = transform.rotation * Vec3::new(0., -torque, 0.);
        }
    }

    if keyboard_input.pressed(KeyCode::S) {
        for (mut forces, _joint, transform, _) in wheels.iter_mut() {
            forces.torque = transform.rotation * Vec3::new(0., torque, 0.);
        }
    }

    if keyboard_input.just_released(KeyCode::W) {
        for (mut forces, _joint, _transform, _) in wheels.iter_mut() {
            forces.torque = Vec3::ZERO;
        }
    }

    if keyboard_input.just_released(KeyCode::S) {
        for (mut forces, _joint,_transform, _) in wheels.iter_mut() {
            forces.torque = Vec3::ZERO;
        }
    }

    let steering_angle = 0.4;
    if keyboard_input.pressed(KeyCode::D) {
        for (_forces, mut joint, _transform, _) in wheels.iter_mut() {
            joint.data.set_local_axis1(Vec3::new(1., 0., steering_angle));
        }
    }

    if keyboard_input.pressed(KeyCode::A) {
        for (_forces, mut joint, _transform, _) in wheels.iter_mut() {
            joint.data.set_local_axis1(Vec3::new(1., 0., -steering_angle));
        }
    }

    if keyboard_input.just_released(KeyCode::D) || keyboard_input.just_released(KeyCode::A) {
        for (_forces, mut joint, _transform, _) in wheels.iter_mut() {
            joint.data.set_local_axis1(Vec3::X);
        }
    }
}

//shout out to https://github.com/bevy-vehicles-wg/bevy-vehicle-template
//for helping me figure out the suspension joints
fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    my_mesh_assets: Res<MyMeshAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //rover properties
    let rover_length = 1.0;
    let rover_width = 0.5;
    let rover_height= 0.25;
    let wheel_radius = 0.2;
    let wheel_width = 0.1;
    let suspension_height = 0.15;
    let starting_height = 1.0;

    let rover_body_collider = (
        Collider::cuboid(
            rover_width / 2.0,
            rover_height / 2.0,
            rover_length / 2.0,
        ),
        ColliderMassProperties::Mass(50.0),
    );

    let camera = Camera3dBundle {
        transform: Transform::from_xyz(0., 2., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    let rover_body = shape::Box {
        min_x: -rover_width / 2.0,
        max_x: rover_width  / 2.0,
        min_y: -rover_height / 2.0,
        max_y: rover_height / 2.0,
        min_z: -rover_length / 2.0,
        max_z: rover_length / 2.0,
    };
    
    let rover_body_entity = commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(rover_body)),
                material: materials.add(Color::GRAY.into()),
                transform: Transform::from_xyz(0.0, starting_height, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            Player,
            rover_body_collider,
        )).with_children(|children| {
            children.spawn(camera);
        }).id();

    let front_left_tform = Vec3::new(-rover_width, -rover_height, rover_length);
    let front_right_tform = Vec3::new(rover_width, -rover_height, rover_length);
    let rear_left_tform = Vec3::new(-rover_width, -rover_height, -rover_length);
    let rear_right_tform = Vec3::new(rover_width, -rover_height, -rover_length);
    let wheel_tforms = vec![
        front_left_tform,
        front_right_tform,
        rear_left_tform,
        rear_right_tform,
    ];

    for (i, wheel_tform) in wheel_tforms.into_iter().enumerate() {
        let suspension_anchor1 = Vec3::new(wheel_tform.x / 2.0, 0.0, wheel_tform.z);
        let suspension_anchor2 = Vec3::new(0.0, -wheel_tform.y, 0.0);
        let suspension_stiffness = 100.0;
        let suspension_damping = 100.0;
        let suspension_joint = PrismaticJointBuilder::new(Vec3::Y)
            .local_anchor1(suspension_anchor1)
            .local_anchor2(suspension_anchor2)
            .limits([-suspension_height / 2.0, suspension_height / 2.0])
            .motor_position(0., suspension_stiffness, suspension_damping);

        let suspension_collider = (
            Collider::cuboid(0.15, 0.05, 0.025),
            ColliderMassProperties::Mass(30.0),
        );

        //attach suspension joint to rover body
        let suspension_entity = commands
            .spawn((
                TransformBundle::from(Transform::from_xyz(wheel_tform.x / 2.0, wheel_tform.y + starting_height, wheel_tform.z)),
                RigidBody::Dynamic,
                suspension_collider,
                ImpulseJoint::new(rover_body_entity, suspension_joint),
            ))
            .id();

        //todo: use RevoluteJointBuilder instead
        //      for some reason RevoluteJointBuilder::new(Vec3::X) doesn't work
        let wheel_joint = GenericJointBuilder::new(JointAxesMask::LOCKED_REVOLUTE_AXES)
            .local_anchor1(Vec3::new(wheel_tform.x / 2.0, 0.0, 0.0))
            .local_axis1(Vec3::X)
            .local_axis2(Vec3::Y)
            .set_motor(JointAxis::AngX, 0.0, 0.0, 0., 1.0);
        // let wheel_joint = RevoluteJointBuilder::new(Vec3::X)
            // .local_anchor1(Vec3::new(wheel_tform.x / 2.0, 0.0, 0.0));

        let wheel_cylinder = shape::Cylinder {
            height: wheel_width, //bc it's rotated 90 deg
            radius: wheel_radius,
            ..default()
        };

        let wheel_collider = (
            Collider::cylinder(wheel_width / 2.0, wheel_radius),
            Friction::coefficient(1.0),
            Restitution::coefficient(0.7),
            ColliderMassProperties::Mass(10.0),
        );

        let mut wheel_entity = commands.spawn((
            RigidBody::Dynamic,
            PbrBundle {
                mesh: meshes.add(Mesh::from(wheel_cylinder)),
                material: materials.add(Color::BLACK.into()),
                transform: Transform {
                    translation: Vec3::new(wheel_tform.x, wheel_tform.y + starting_height, wheel_tform.z),
                    rotation: Quat::from_rotation_z(-90_f32.to_radians()),
                    scale: Vec3::ONE,
                },
                ..default()
            },
            wheel_collider,
            ExternalForce::default(),
            ImpulseJoint::new(suspension_entity, wheel_joint),
        ));

        if i > 1 {
            wheel_entity.insert(FrontWheel);
        }
    }
}