use bevy::{
    light::CascadeShadowConfigBuilder,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    prelude::*
};

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<SkyMaterial>::default());
        app.add_systems(
            Startup,
            (
                spawn_sun,
                spawn_sky,
            )
        );
    }
}

fn spawn_sun(mut commands: Commands) {
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: 3.0,
        ..default()
    }.build();

    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.98, 0.95, 0.82),
            shadow_maps_enabled: true,
            ..default()
        },
        Transform:: from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
        cascade_shadow_config,
    ));
}

#[derive(Component)]
struct Sky;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SkyMaterial {
    #[uniform(0)]
    pub top_color: LinearRgba,

    #[uniform(1)]
    pub horizon_color: LinearRgba,
}

impl Material for SkyMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/sky.wgsl".into()
    }
}

fn spawn_sky(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SkyMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2000.0, 2000.0, 2000.0))),
        MeshMaterial3d(materials.add(SkyMaterial {
            top_color: LinearRgba::rgb(0.02, 0.08, 0.3),
            horizon_color: LinearRgba::rgb(0.5, 0.75, 1.0)
        })),
        Sky,
    ));
}