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
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
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
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(Msaa::Sample8)
        .add_systems(Startup, (setup,add_colliders))
        .add_systems(Update, (cast_ray,add_colliders))
        .add_event::<NewTextEntry>()
        .run();
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {


    // Load Floor
    let myscene = asset_server.load("Floor.gltf#Scene0");
    println!("{:?}",myscene);
    commands.spawn((SceneBundle {
        scene: myscene,
        transform: Transform::from_xyz(0.0, -4.0, 4.0),
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
    mut hit_indices: ResMut<HitIndices>,
) {

    hit_indices.hits = Vec::new(); 
    for x in 0..101 {
        for y in 0..25 { 
            let x_loc:f32 = -2.0 + (x as f32)*0.04;
            let y_loc:f32 = 0.65 - (y as f32)*0.03;

            let ray_pos = Vec3::new(x_loc,y_loc,-1.0);
            let ray_dir = Vec3::new(0.0, 0.0, 1.0);
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
                hit_indices.hits.push((x,y));
                let hit_point = intersection.point;
                let hit_normal = intersection.normal;
                //println!("Entity {:?} hit at point {} with normal {}", entity, hit_point, hit_normal);
                false // Return `false` instead if we want to stop searching for other hits.
            });

        }
    }
    
    
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





