use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod perlin_noise;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (setup_physics, spawn_light_source))
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .insert_resource(ClearColor(Color::rgb(0., 0., 0.)));
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

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_mesh = create_noise_map_mesh();
    let ground_mesh_collider = Collider::from_bevy_mesh(&ground_mesh, &ComputedColliderShape::TriMesh).unwrap();
    let floor = PbrBundle {
        mesh: meshes.add(ground_mesh),
        material: materials.add(Color::BEIGE.into()),
        ..default()
    };
    
    commands
        .spawn(floor)
        .insert(ground_mesh_collider)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)
                                                        .with_scale(Vec3::new(5., 1., 5.))));
}

//https://lejondahl.com/heightmap/
fn create_noise_map_mesh() -> Mesh {
    let amplitude: f64 = 3.0;
    let width: usize = 32;
    let height: usize = 32;

    let num_vertices: usize = width * height;
    let vertices_per_triangle = 3;
    let triangles_per_square = 2;
    let num_triangles: usize = width * height * triangles_per_square * vertices_per_triangle;

    let (width_u32, height_u32) = (width as u32, height as u32);
    let (width_f32, height_f32) = (width as f32, height as f32);
    let amplitude_f32 = amplitude as f32;

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices);

    let perlin = perlin_noise::PerlinNoise::new();
    let amplitude = 1.;
    let freq = 1.;
    let octaves = 8;
    let gain = 0.5;
    let lacunarity = 1.92;
    for h in 0..=height {
        for w in 0..=width {
            let x = h as f32 / width as f32;
            let y = w as f32 / height as f32;
            let (i_f32, j_f32) = (h as f32, w as f32);

            let mut val = 0.;
            let mut a = amplitude;
            let mut f = freq;
            for _ in 0..octaves {
                val += a * perlin.gen_2d_noise(x * f, y * f);
                a *= gain;
                f *= lacunarity;
            }

            let pos = [
                (i_f32 - width_f32 / 2.) * amplitude_f32 / width_f32,
                val,
                (j_f32 - height_f32 / 2.) * amplitude_f32 / height_f32,
            ];
            positions.push(pos);
            normals.push([0.0, 1.0, 0.0]);
            uvs.push([x, y]);
        }
    }

    // Defining triangles.
    let mut triangles: Vec<u32> = Vec::with_capacity(num_triangles);

    for h in 0..height_u32 {
        for w in 0..width_u32 {
            triangles.push((h * (width_u32 + 1)) + w);
            triangles.push(((h + 1) * (width_u32 + 1)) + w);
            triangles.push(((h + 1) * (width_u32 + 1)) + w + 1);

            triangles.push((h * (width_u32 + 1)) + w);
            triangles.push(((h + 1) * (width_u32 + 1)) + w + 1);
            triangles.push((h * (width_u32 + 1)) + w + 1);
        }
    }

    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(triangles)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh


}
