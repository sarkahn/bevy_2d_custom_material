mod color_material_test;
mod custom_pipeline_material;

use custom_pipeline_material::*;
use color_material_test::*;

use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}, sprite::Mesh2dHandle};

/// This example shows how to render 2d items using a custom pipeline for 2d meshes
/// It doesn't use the [`Material2d`] abstraction, but changes the vertex buffer to include vertex color
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CustomMaterialPlugin)
        //.add_plugin(ColorMaterialTestPlugin)
        .add_startup_system(spawn_custom_material)
        .run();
}

fn spawn_custom_material(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // We will add a new Mesh for the star being created
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let size = 50.0_f32;
    let verts = vec![
        [0.0,size,0.0],
        [0.0,0.0,0.0],
        [size,size,0.0],
        [size,0.0,0.0],
    ];
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, verts);

    let indices = vec![0,1,2,3,2,1];
    mesh.set_indices(Some(Indices::U32(indices)));

    let uv = vec![
        [0.0,0.0],
        [0.0,1.0],
        [1.0,0.0],
        [1.0,1.0],
    ];
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uv);

    // let normals = vec![[0.0,0.0,-1.0]; 4];
    // mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    let mesh = Mesh2dHandle(meshes.add(mesh));

    let material = materials.add(CustomMaterial {
        color: Color::BLUE,
        texture: None,//Some(asset_server.load("alien.png")),
    });

    // We can now spawn the entities for the star and the camera,
    // this is all the components needed for a 2d entity to render.
    commands.spawn_bundle(CustomMaterialBundle {
        material,
        mesh,
        ..Default::default()
    });

    commands
        // And use an orthographic projection
        .spawn_bundle(OrthographicCameraBundle {
            orthographic_projection: OrthographicProjection {
                //scale: 0.05,
                ..Default::default()
            },
            ..OrthographicCameraBundle::new_2d()
        });
}