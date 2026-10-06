#[derive(Asset, TypePath, AsBindGroup, Clone)]
#[bind_group_data(DynShaderKey)]
pub struct DynMaterial {
    pub shader: Handle<Shader>,
    pub alpha: AlphaMode2d,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DynShaderKey {
    shader: Handle<Shader>,
}

impl From<&DynMaterial> for DynShaderKey {
    fn from(m: &DynMaterial) -> Self {
        Self { shader: m.shader.clone() }
    }
}

impl Material2d for DynMaterial {
    fn alpha_mode(&self) -> AlphaMode2d {
        self.alpha
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(fragment) = descriptor.fragment.as_mut() {
            fragment.shader = key.bind_group_data.shader.clone();
        }
        Ok(())
    }
}

#[derive(Resource, Default)]
pub struct ShaderRegistry {
    
}