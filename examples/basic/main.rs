use bevy::{prelude::*, render::mesh::Indices};
use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};
use scripted_mesh_experiment::ScriptedMeshEngine;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PanOrbitCameraPlugin))
        .add_systems(Startup, setup)
        // .add_systems(Update, update)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, PanOrbitCamera::default()));

    // generate test mesh

    // let shapes = read_shapes_from_bin_file("./assets/test.amb");
    // let shapes = 
    //     if shapes.is_ok() { shapes.unwrap() }
    //     else { panic!("Load error: {:?}", shapes.err().unwrap()); };

    let engine = ScriptedMeshEngine::new();
    let shape = engine.from_path("./examples/basic/basic.rhai");
    let info = if shape.is_ok() { shape.unwrap() } else { panic!("Failed to load rhai mesh! {:?}", shape.err().unwrap()); };

    // load each individual shape
    // for shape in shapes {
        // generate shape info and unpack
        // let info = gen_shape_mesh(&shape);
        let positions = info.positions;
        let normals = info.normals;
        
        // generate mesh and update values
        let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_indices(Some(Indices::U32(info.indices)));

        // temp normals
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

        // spawn mesh
        commands.spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            ..default()
        });
    // }
}