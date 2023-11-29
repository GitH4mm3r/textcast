mod camera;
mod light_array;
mod text_input;

use light_array::*;
use text_input::*;
use camera::*; 
use bevy::{
    prelude::*,
    window::PrimaryWindow,
    reflect::TypePath,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    render::{
        RenderPlugin,
        mesh::{MeshVertexBufferLayout, PrimitiveTopology},
        settings::{WgpuSettings,Backends},
        render_resource::{
            AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};
use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_rapier3d::prelude::*;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::f32::consts::*;


fn main() {

    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: WgpuSettings {
                    backends: Some(Backends::VULKAN),
                    ..default()
                }
                .into(),
            }),
            RapierPhysicsPlugin::<NoUserData>::default(),
            LightArrayPlugin,
            CameraPlugin,
            //WorldInspectorPlugin::new(),
            TextinPlugin,
            
        ))
        .insert_resource(Msaa::Sample8)
        .add_systems(Startup, (setup,add_colliders))
        .add_systems(Update, (shift_left,cast_ray,add_colliders))
        .add_event::<NewTextEntry>()
        .run();
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {


    // let myscene = asset_server.load("F.gltf#Scene0");
    // println!("{:?}",myscene);
    // commands.spawn((SceneBundle {
    //     scene: myscene,
    //     transform: Transform::from_xyz(3.0, 0.0, 0.1),
    //     ..default()
    //     }, ScrollingTextEnt,)
    // );

    // Load Floor
    let myscene = asset_server.load("Floor.gltf#Scene0");
    println!("{:?}",myscene);
    commands.spawn((SceneBundle {
        scene: myscene,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
        })
    );
   
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_seconds() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}

fn shift_left(mut query: Query<&mut Transform, With<ScrollingTextEnt>>, time: Res<Time>, mut text: Query<&mut Text, With<MyText>>) {
    let rotation = time.delta_seconds() / 2.;
    for mut transform in &mut query {
        //transform.rotate_local_x(time.delta_seconds() * PI / 4.0);
        transform.translation.x -= 0.4 * time.delta_seconds();
        if transform.translation.x < -3.0 {
            transform.translation.x = 3.0;
        }
    }
}

fn add_colliders(
    mut commands: Commands,
    scene_meshes: Query<(Entity, &Name, &Handle<Mesh>), Added<Name>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // iterate over all meshes in the scene and match them by their name.
    for (entity, name, mesh_handle) in scene_meshes.iter() {
        // "LetterA" would be the name of the Letter object in Blender.
        if name.as_str() == "Text" {
            let mesh = meshes.get(mesh_handle).unwrap();
            // Create the collider from the mesh.
            let collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();
            println!("added collider to mesh {:?}",entity);
            // Attach collider to the entity of this same object.
            commands
                .entity(entity)
                .insert(collider);
                
        }
    }
}

fn cast_ray(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    rapier_context: Res<RapierContext>,
) {

        let ray_pos = Vec3::new(-0.650,0.25,1.0);
        let ray_dir = Vec3::new(0.0, 0.0, -1.0);
        let max_toi = f32::MAX;
        let solid = true; 
        let filter = QueryFilter::new();

        // Then cast the ray.
        let hit = rapier_context.cast_ray(
            ray_pos, 
            ray_dir,
            max_toi,
            true,
            filter,
        );

        rapier_context.intersections_with_ray(
            ray_pos, ray_dir, max_toi, solid, filter,
            |entity, intersection| {
                // Callback called on each collider hit by the ray.
                let hit_point = intersection.point;
                let hit_normal = intersection.normal;
                println!("Entity {:?} hit at point {} with normal {}", entity, hit_point, hit_normal);
                true // Return `false` instead if we want to stop searching for other hits.
            });
    
}

fn display_et (
    mut disp_text: Query<(Entity, &mut Text), With<DispText>>,
    mut entered_text: ResMut<EnteredText>
){
    let (ent,mut text) = disp_text.single_mut();
    text.sections[0].value = entered_text.text.clone();
}

#[derive(Component)]
struct DispText;

#[derive(Component)]
struct MyText;

#[derive(Component)]
struct ScrollingTextEnt;



