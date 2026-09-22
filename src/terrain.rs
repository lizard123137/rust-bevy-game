use bevy::{
    mesh::Indices,
    asset::RenderAssetUsages,
    render::render_resource::PrimitiveTopology,
    prelude::*
};
use noise::{NoiseFn, Perlin};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_terrain);
    }
}

fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = generate_terrain();
    let mesh_handle = meshes.add(mesh);

    let material_handle = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 1.0,
        ..default()
    });

    commands.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(material_handle),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(
            Quat::from_euler(
                EulerRot::XYZ,
                -1.0,
                -0.5,
                0.0,
            ),
        ),
    ));
}

const SIZE: usize = 128;
const CELL_SIZE: f32 = 5.0;
const HEIGHT: f32 = 50.0;
const NOISE_SCALE: f64 = 0.01;

fn generate_terrain() -> Mesh {
    let noise = Perlin::new(12345);

    let vertices_per_side = SIZE + 1;
    
    let mut positions = Vec::with_capacity(
        vertices_per_side * vertices_per_side
    );

    let mut normals = Vec::with_capacity(
        vertices_per_side * vertices_per_side
    );

    let mut uvs = Vec::with_capacity(
        vertices_per_side * vertices_per_side
    );

    let mut colors = Vec::with_capacity(
        vertices_per_side * vertices_per_side
    );

    for z in 0..=SIZE {
        for x in 0..=SIZE {
            let world_x = x as f32 * CELL_SIZE;
            let world_z = z as f32 * CELL_SIZE;

            let height = get_height(
                world_x,
                world_z,
                &noise,
            );

            let color = terrain_color(height);
            colors.push(color.to_linear().to_f32_array());

            positions.push([
                world_x,
                height,
                world_z
            ]);

            uvs.push([
                x as f32 / SIZE as f32,
                z as f32 / SIZE as f32,
            ]);

            normals.push([0.0, 1.0, 0.0]);
        }
    }

    let mut indices = Vec::with_capacity(SIZE * SIZE * 6);

    for z in 0..SIZE {
        for x in 0..SIZE {
            let a = z * vertices_per_side + x;
            let b = a + 1;
            let c = a + vertices_per_side;
            let d = c + 1;

            indices.extend_from_slice(&[
                a as u32,
                c as u32,
                b as u32,

                b as u32,
                c as u32,
                d as u32,
            ]);
        }
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        positions,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        normals,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        uvs,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_COLOR,
        colors,
    )
    .with_inserted_indices(
        Indices::U32(indices),
    )
}

fn get_height(
    x: f32,
    z: f32,
    noise: &Perlin,
) -> f32 {
    noise.get([
        x as f64 * NOISE_SCALE,
        z as f64 * NOISE_SCALE,
    ]) as f32 * HEIGHT
}

fn terrain_color(height: f32) -> Color {
    if height < 5.0 {
        Color::srgb(0.76, 0.70, 0.45)
    } else if height < 15.0 {
        Color::srgb(0.20, 0.50, 0.12)
    } else if height < 30.0 {
        Color::srgb(0.12, 0.35, 0.08)
    } else if height < 40.0 {
        Color::srgb(0.35, 0.30, 0.20)
    } else {
        Color::WHITE
    }
}