use bevy::prelude::*;
use bevy::{input::keyboard::KeyboardInput, prelude::*};


pub struct TextinPlugin;

impl Plugin for TextinPlugin { 
    fn build(&self, app:&mut App){ 
        app.add_systems(Update,read_input);
    }
}

fn read_input(keyboard_input: Res<Input<KeyCode>>) {
    
    if keyboard_input.pressed(KeyCode::A) {
        info!("'A' currently pressed");
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        info!("'A' just pressed");
    }

    if keyboard_input.just_released(KeyCode::A) {
        info!("'A' just released");
    }

}