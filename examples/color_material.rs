use bevy::{
    prelude::*,
    render::{
        mesh::Indices, render_resource::PrimitiveTopology,
    }, sprite::{Mesh2dHandle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_color_material)
        .run();
}

// Demonstrates how to use `ColorMesh2dBundle`, for reference
fn spawn_color_material(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut quad = Mesh::new(PrimitiveTopology::TriangleList);

    let size = 24.0_f32 * 5.0;

    // Create a quad with a bottom-left pivot.
    let verts = vec![
        [0.0,size,0.0],
        [0.0,0.0,0.0],
        [size,size,0.0],
        [size,0.0,0.0],
    ];
    
    quad.set_attribute(Mesh::ATTRIBUTE_POSITION, verts);

    let indices = vec![0,1,2,3,2,1];
    quad.set_indices(Some(Indices::U32(indices)));

    let uv = vec![
        [0.0,0.0],
        [0.0,1.0],
        [1.0,0.0],
        [1.0,1.0],
    ];
    quad.set_attribute(Mesh::ATTRIBUTE_UV_0, uv);

    // As we can't yet customize Vertex Attributes, we must provide normals 
    // when using ColorMaterial, even if they're not used.
    let normals =  vec![[0.0,0.0,1.0]; 4];
    quad.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    let mesh = Mesh2dHandle(meshes.add(quad));

    let image = asset_server.load("alien.png");
    let material = materials.add(ColorMaterial::from(image));

    commands.spawn_bundle(ColorMesh2dBundle {
        mesh,
        material,
        ..Default::default()
    });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
