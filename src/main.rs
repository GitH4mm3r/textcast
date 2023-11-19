mod camera;


use camera::*; 
use bevy::{
    prelude::*,
    window::PrimaryWindow,
    reflect::TypePath,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    render::{
        mesh::{MeshVertexBufferLayout, PrimitiveTopology},
        render_resource::{
            AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::f32::consts::*;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<LineMaterial>::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            //RapierDebugRenderPlugin::default(),
            CameraPlugin,
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, (setup,add_colliders))
        .add_systems(Update, (animate_light_direction,rotate,cast_ray,add_colliders))
        .run();
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<LineMaterial>>, 
    mut meshes: ResMut<Assets<Mesh>>,
) {

    // ...spawn colliders and other things 
    let myscene = asset_server.load("A.gltf#Scene0");
    println!("{:?}",myscene);
    commands.spawn((SceneBundle {
        scene: myscene,
        transform: Transform::from_xyz(-1.0, 0.0, 0.1),
        ..default()
        }, MyScene,)
    );
   
    // Spawn a list of lines with start and end points for each lines
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(LineList {
            lines: vec![
                (Vec3::new(-0.650, 0.25, 1.0), Vec3::new(-0.650, 0.250, -1.0)),
            ],
        })),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(LineMaterial {
            color: Color::GREEN,
        }),
        ..default()
    });
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

fn rotate(mut query: Query<&mut Transform, With<MyScene>>, time: Res<Time>) {
    let rotation = time.delta_seconds() / 2.;
    for mut transform in &mut query {
        transform.rotate_local_x(time.delta_seconds() * PI / 4.0);
        //transform.rotate_local_y(time.delta_seconds() * PI / 4.0);
        
        
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
            println!("added mesh");
            // Create the collider from the mesh.
            let collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();
            println!("added mesh");
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

        if let Some((entity, _toi)) = hit {
            // Color in blue the entity we just hit.
            // Because of the query filter, only colliders attached to a dynamic body
            // will get an event.
            let color = Color::BLUE;
            println!("hit!");
            commands.entity(entity).insert(ColliderDebugColor(color));
        }

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



#[derive(Component)]
struct MyScene;

#[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone)]
struct LineMaterial {
    #[uniform(0)]
    color: Color,
}

/// A list of points that will have a line drawn between each consecutive points
#[derive(Debug, Clone)]
pub struct LineStrip {
    pub points: Vec<Vec3>,
}

/// A list of lines with a start and end position
#[derive(Debug, Clone)]
pub struct LineList {
    pub lines: Vec<(Vec3, Vec3)>,
}

impl From<LineStrip> for Mesh {
    fn from(line: LineStrip) -> Self {
        // This tells wgpu that the positions are a list of points
        // where a line will be drawn between each consecutive point
        Mesh::new(PrimitiveTopology::LineStrip)
            // Add the point positions as an attribute
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, line.points)
    }
}

impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        let vertices: Vec<_> = line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();

        // This tells wgpu that the positions are list of lines
        // where every pair is a start and end point
        Mesh::new(PrimitiveTopology::LineList)
            // Add the vertices positions as an attribute
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    }
}

impl Material for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/line_material.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // This is the important part to tell bevy to render this material as a line between vertices
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}
