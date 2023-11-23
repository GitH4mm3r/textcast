use bevy::prelude::*; 
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app:&mut App){
        app.add_systems(Startup, spawn_camera)
        .insert_resource(DirectionalLightShadowMap { size: 4096 });
    }
}

fn spawn_camera( mut commands: Commands,asset_server: Res<AssetServer>,) {

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 3.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        },
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        // This is a relatively small scene, so use tighter shadow
        // cascade bounds than the default for better quality.
        // We also adjusted the shadow map to be larger since we're
        // only using a single cascade.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .into(),
        ..default()
    });

}

