use bevy::app::{App, Plugin};
use bevy::asset::{AssetServer, Assets, Handle, HandleUntyped};
use bevy::ecs::system::{lifetimeless::SRes, SystemParamItem};
use bevy::math::Vec4;
use bevy::reflect::TypeUuid;
use bevy::render::{
    color::Color,
    prelude::Shader,
    render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
    render_resource::{
        std140::{AsStd140, Std140},
        *,
    },
    renderer::RenderDevice,
    texture::Image,
};

use bevy::sprite::{Material2d, Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle, SpecializedMaterial2d};

pub const CUSTOM_MATERIAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 3142086872234592509);

#[derive(Default)]
pub struct CustomMaterialPlugin;

impl Plugin for CustomMaterialPlugin {
    fn build(&self, app: &mut App) {
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        shaders.set_untracked(
            CUSTOM_MATERIAL_SHADER_HANDLE,
            Shader::from_wgsl(include_str!("shader.wgsl")),
        );

        app.add_plugin(Material2dPlugin::<CustomMaterial>::default());

        app.world
            .get_resource_mut::<Assets<CustomMaterial>>()
            .unwrap()
            .set_untracked(
                Handle::<CustomMaterial>::default(),
                CustomMaterial {
                    color: Color::rgb(1.0, 0.0, 1.0),
                    ..Default::default()
                },
            );
    }
}

/// A [2d material](Material2d) that renders [2d meshes](crate::Mesh2dHandle) with a texture tinted by a uniform color
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "e228a534-e3ca-2e1e-ab9d-4d8bc1ad8c19"]
pub struct CustomMaterial {
    pub color: Color,
    pub texture: Option<Handle<Image>>,
}

impl Default for CustomMaterial {
    fn default() -> Self {
        CustomMaterial {
            color: Color::rgb(1.0, 0.0, 1.0),
            texture: None,
        }
    }
}

impl From<Color> for CustomMaterial {
    fn from(color: Color) -> Self {
        CustomMaterial {
            color,
            ..Default::default()
        }
    }
}

impl From<Handle<Image>> for CustomMaterial {
    fn from(texture: Handle<Image>) -> Self {
        CustomMaterial {
            texture: Some(texture),
            color: Color::WHITE,
        }
    }
}

// NOTE: These must match the bit flags in bevy_sprite/src/mesh2d/color_material.wgsl!
bitflags::bitflags! {
    #[repr(transparent)]
    pub struct ColorMaterialFlags: u32 {
        const TEXTURE           = (1 << 0);
        const NONE              = 0;
        const UNINITIALIZED     = 0xFFFF;
    }
}

/// The GPU representation of the uniform data of a [`ColorMaterial`].
#[derive(Clone, Default, AsStd140)]
pub struct CustomMaterialUniformData {
    pub color: Vec4,
    pub flags: u32,
}

/// The GPU representation of a [`ColorMaterial`].
#[derive(Debug, Clone)]
pub struct GpuCustomMaterial {
    /// A buffer containing the [`ColorMaterialUniformData`] of the material.
    pub buffer: Buffer,
    /// The bind group specifying how the [`ColorMaterialUniformData`] and
    /// the texture of the material are bound.
    pub bind_group: BindGroup,
    pub flags: ColorMaterialFlags,
    pub texture: Option<Handle<Image>>,
}

impl RenderAsset for CustomMaterial {
    type ExtractedAsset = CustomMaterial;
    type PreparedAsset = GpuCustomMaterial;
    type Param = (
        SRes<RenderDevice>,
        SRes<Material2dPipeline<CustomMaterial>>,
        SRes<RenderAssets<Image>>,
    );

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        material: Self::ExtractedAsset,
        (render_device, color_pipeline, gpu_images): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let (texture_view, sampler) = if let Some(result) = color_pipeline
            .mesh2d_pipeline
            .get_image_texture(gpu_images, &material.texture)
        {
            result
        } else {
            return Err(PrepareAssetError::RetryNextUpdate(material));
        };

        let mut flags = ColorMaterialFlags::NONE;
        if material.texture.is_some() {
            flags |= ColorMaterialFlags::TEXTURE;
        }

        let value = CustomMaterialUniformData {
            color: material.color.as_linear_rgba_f32().into(),
            flags: flags.bits(),
        };
        let value_std140 = value.as_std140();

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("color_material_uniform_buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: value_std140.as_bytes(),
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(texture_view),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(sampler),
                },
            ],
            label: Some("color_material_bind_group"),
            layout: &color_pipeline.material2d_layout,
        });

        Ok(GpuCustomMaterial {
            buffer,
            bind_group,
            flags,
            texture: material.texture,
        })
    }
}

impl Material2d for CustomMaterial {
    fn fragment_shader(_asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(CUSTOM_MATERIAL_SHADER_HANDLE.typed())
    }

    #[inline]
    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(
        render_device: &RenderDevice,
    ) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            CustomMaterialUniformData::std140_size_static() as u64,
                        ),
                    },
                    count: None,
                },
                // Texture
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                // Texture Sampler
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("color_material_layout"),
        })
    }

    // type Key =  ();

    // fn key(material: &<Self as RenderAsset>::PreparedAsset) -> Self::Key {
    //     ()
    // }

    // fn specialize(key: Self::Key, descriptor: &mut RenderPipelineDescriptor) {
        
    // }
}

/// A component bundle for entities with a [`Mesh2dHandle`](crate::Mesh2dHandle) and a [`ColorMaterial`].
pub type CustomMaterialBundle = MaterialMesh2dBundle<CustomMaterial>;
