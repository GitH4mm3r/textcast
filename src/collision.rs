use bevy::prelude::*; 
use bevy_rapier3d::prelude::*;

pub Struct CollisionPlugin;

impl plugin for CollisionPlugin{
    fn build(&self,app:&mut App){
        app.add_systems(Update, ray_cast);
    }
}