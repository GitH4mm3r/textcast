use bevy::prelude::*; 
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use std::f32::consts::PI;
use bevy::render::view::Visibility::*;

pub struct LightArrayPlugin;

impl Plugin for LightArrayPlugin {
    fn build(&self, app:&mut App){
        app.add_systems(Startup, setup_lights)
        .add_systems(Update, control_lights)
        .insert_resource(HitIndices{hits:Vec::new()});
        //.insert_resource(DirectionalLightShadowMap { size: 1024 });
    }
}

//lets create a 120x40 array of led lights and go from x=-2 to 2 and y 0 to 1.5
// for x in -2 to 2

fn setup_lights(  mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {

    let mesheses = meshes.add(Mesh::from(shape::UVSphere{
        radius: 0.01,
        sectors: 4,
        stacks: 4,
        ..default()
    }));

    let materialsest = materials.add(StandardMaterial {
        base_color: Color::GREEN,
        emissive: Color::rgb_linear(5.99, 0.0, 0.0),
       // emissive: Color::rgba_linear(5.0, 1.13, 1.0, 0.0),
        ..default()
    });


    let materialses = materials.add(StandardMaterial {
        base_color: Color::GREEN,
        emissive: Color::rgb_linear(5.99, 0.0, 0.0),
       // emissive: Color::rgba_linear(5.0, 1.13, 1.0, 0.0),
        ..default()
    });


    let point_lights = PointLight {
        intensity: 0.50, // lumens - roughly a 100W non-halogen incandescent bulb
        color: Color::RED,
        shadows_enabled: false,
        ..default()
    };

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb_u8(0, 0, 0).into()),
        transform: Transform::from_xyz(0.0, 0.40, -1.2).with_scale(Vec3::new(4.4,2.0,0.20)),
        ..default()
    });


    for x in 0..101 {
        for y in 0..25 { 
            let x_loc:f32 = -2.0 + (x as f32)*0.04;
            let y_loc:f32 = 0.65 - (y as f32)*0.03;

            commands.spawn((PbrBundle {
                    mesh: mesheses.clone(),
                    material: materialses.clone(),
                    transform: Transform::from_xyz(x_loc, y_loc, -1.0),
                    ..default()
                },Light{x:x,y:y}));
            
        }
    }
}

fn control_lights( mut commands: Commands,
    asset_server: Res<AssetServer>, 
    hit_indices: Res<HitIndices>, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut lights: Query<(Entity,&mut Handle<StandardMaterial>, &Light, &mut Visibility)>
) {

    for (entity,mat_handle,light,mut visible) in &mut lights { 

    
        if hit_indices.hits.contains(&(light.x,light.y)){ 
            *visible=Visible;
        }
        else { 
            *visible=Hidden;
        }

    }


}

#[derive(Component)]
struct Light{
    x:u32,
    y:u32,
}

#[derive(Resource)]
pub struct HitIndices {
   pub hits: Vec<(u32,u32)>,
}