use bevy::{
    camera::visibility::RenderLayers,
    ecs::system::SystemParam,
    prelude::*
};

use crate::graphics::effects::{
    fog::FogMaterial,
    fire::FireMaterial,
};

#[derive(SystemParam)]
pub struct ShaderMaterials<'w> {
    pub fog: ResMut<'w, Assets<FogMaterial>>,
    pub fire: ResMut<'w, Assets<FireMaterial>>,
    // pub water: ResMut<'w, Assets<WaterMaterial>>,
}

pub fn spawn_shader(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut ShaderMaterials,
    asset_server: &AssetServer,
    pos: Vec2,
    name: &str,
    size: Vec2,
) {
    let mut entity = commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(size.x, size.y))),
        Transform::from_xyz(pos.x, pos.y, 0.0),
        RenderLayers::layer(1),
    ));

    match name {
        "fog" => {
            entity.insert(MeshMaterial2d(materials.fog.add(FogMaterial {})));
        }
        "fire" => {
            entity.insert(MeshMaterial2d(materials.fire.add(FireMaterial {})));
        }
        _ => {
            warn!("Unknown material '{name}'");
            entity.despawn();
        }
    };
}