use bevy_2d_mesh_example::*;

use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}, sprite::Mesh2dHandle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CustomMaterialPlugin)
        .add_startup_system(spawn_custom_material)
        .run();
}

// Demonstrates to use a custom 2d material
fn spawn_custom_material(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);


    let size = 24.0_f32 * 5.0;
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

    let colors: Vec<[f32;4]> = vec![
        Color::RED.into(),
        Color::BLUE.into(),
        Color::GREEN.into(),
        Color::YELLOW.into(),
    ];
    mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, colors);

    let mesh = Mesh2dHandle(meshes.add(mesh));

    let material = materials.add(CustomMaterial {
        color: Color::WHITE,
        texture: Some(asset_server.load("alien.png")),
    });

    commands.spawn_bundle(CustomMaterialBundle {
        material,
        mesh,
        ..Default::default()
    });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}