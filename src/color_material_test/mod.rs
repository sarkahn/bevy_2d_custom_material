use bevy::{
    prelude::*,
    render::{
        mesh::Indices, render_resource::PrimitiveTopology,
    }, sprite::{Mesh2dHandle},
};

pub struct ColorMaterialTestPlugin;

impl Plugin for ColorMaterialTestPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_color_material);
    }
}

fn spawn_color_material(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // We will add a new Mesh for the star being created
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut quad = Mesh::new(PrimitiveTopology::TriangleList);

    let size = 1.0_f32;

    let v_pos = vec![
        [0.0,size,0.0],
        [0.0,0.0,0.0],
        [size,size,0.0],
        [size,0.0,0.0],
    ];
    // Set the position attribute
    quad.set_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    // And a RGB color attribute as well
    let mut v_color = Vec::new();
    v_color.extend_from_slice(&[[1.0, 0.0, 0.0, 1.0]; 4]);

    let indices = vec![0,1,2,3,2,1];
    quad.set_indices(Some(Indices::U32(indices)));

    let uv = vec![
        [0.0,1.0],
        [0.0,0.0],
        [1.0,1.0],
        [1.0,0.0],
    ];
    quad.set_attribute(Mesh::ATTRIBUTE_UV_0, uv);

    let normals =  vec![[0.0,0.0,1.0]; 4];
    quad.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    let mesh = Mesh2dHandle(meshes.add(quad));
    let material = asset_server.load("alien.png").into();

    // We can now spawn the entities for the star and the camera,
    // this is all the components needed for a 2d entity to render.
    commands.spawn_bundle(ColorMesh2dBundle {
        mesh,
        material,
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
