use bevy::prelude::*; 
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use std::f32::consts::PI;

pub struct LightArrayPlugin;

impl Plugin for LightArrayPlugin {
    fn build(&self, app:&mut App){
        app.add_systems(Startup, setup_lights)
        .add_systems(Update, control_lights)
        .insert_resource(DirectionalLightShadowMap { size: 4096 });
    }
}

fn setup_lights(  mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 3.0, 10.0),
        point_light: PointLight {
            intensity: 20000.0, // lumens - roughly a 100W non-halogen incandescent bulb
            color: Color::RED,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    })
    .with_children(|builder| {
        builder.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.1,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::RED,
                emissive: Color::rgba_linear(0.0, 7.13, 0.0, 0.0),
                ..default()
            }),
            ..default()
        });
    });
}
fn control_lights( mut commands: Commands,asset_server: Res<AssetServer>,) {

}