mod camera;


use camera::*; 
use bevy::{
    prelude::*,
    window::PrimaryWindow,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    reflect::TypePath,
    render::{
        mesh::{MeshVertexBufferLayout, PrimitiveTopology},
        render_resource::{
            AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};
use bevy_rapier3d::prelude::*;
use std::f32::consts::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<LineMaterial>::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            CameraPlugin,
        ))
        .add_systems(Startup, (setup,add_colliders))
        .add_systems(Update, (animate_light_direction,rotate,cast_ray,move_scene_entities,add_colliders))
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
        transform: Transform::from_xyz(1.0, 0.0, 0.0),
        ..default()
        }, MyScene,)
    );
   
    // Spawn a list of lines with start and end points for each lines
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(LineList {
            lines: vec![
                (Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0)),
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
        
    }
}

#[derive(Component)]
struct MyScene;

#[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone)]
struct LineMaterial {
    #[uniform(0)]
    color: Color,
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

/// A list of lines with a start and end position
#[derive(Debug, Clone)]
pub struct LineList {
    pub lines: Vec<(Vec3, Vec3)>,
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

/// A list of points that will have a line drawn between each consecutive points
#[derive(Debug, Clone)]
pub struct LineStrip {
    pub points: Vec<Vec3>,
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

fn add_colliders(
    mut commands: Commands,
    scene_meshes: Query<(Entity, &Name, &Handle<Mesh>), Added<Name>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // iterate over all meshes in the scene and match them by their name.
    for (entity, name, mesh_handle) in scene_meshes.iter() {
        // "LetterA" would be the name of the Letter object in Blender.
        if name.to_string() == "Text" {
            let mesh = meshes.get(mesh_handle).unwrap();
            println!("added mesh");
            // Create the collider from the mesh.
            let collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();
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
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // We will color in read the colliders hovered by the mouse.
    for (camera, camera_transform) in &cameras {
        // First, compute a ray from the mouse position.
        let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };

        // Then cast the ray.
        let hit = rapier_context.cast_ray(
            Vec3::ZERO, 
            Vec3::new(1.0, 0.0, 0.0),
            f32::MAX,
            true,
            QueryFilter::new(),
        );

        if let Some((entity, _toi)) = hit {
            // Color in blue the entity we just hit.
            // Because of the query filter, only colliders attached to a dynamic body
            // will get an event.
            let color = Color::BLUE;
            println!("hit!");
            commands.entity(entity).insert(ColliderDebugColor(color));
        }
    }
}

fn move_scene_entities(
    time: Res<Time>,
    moved_scene: Query<Entity, With<MyScene>>,
    children: Query<&Children>,
    mut transforms: Query<&mut Transform>,
) { 
    for moved_scene_entity in &moved_scene {
        let mut offset = 0.;
        for entity in children.iter_descendants(moved_scene_entity) {
            if let Ok(mut transform) = transforms.get_mut(entity) {
                transform.translation = Vec3::new(
                    offset * time.elapsed_seconds().sin() / 20.,
                    0.,
                    time.elapsed_seconds().cos() / 20.,
                );
                offset += 0.5;
            }
        }
    }
}