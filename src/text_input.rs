use bevy::prelude::*;
use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_rapier3d::prelude::*;

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
use bevy::render::view::Visibility::*;
use std::f32::consts::*;

pub struct TextinPlugin;

impl Plugin for TextinPlugin { 
    fn build(&self, app:&mut App){ 
        app.add_systems(Startup,text_setup,)
        .add_systems(Update,(listen_kb_events,listen_rx_char_events,display_et,place_text_mesh,shift_left))
        .insert_resource(EnteredText{text:format!("")});
    }
}

fn text_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {

    // Text: this text displays 
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                color:Color::WHITE,
                font: asset_server.load("pop_warner.ttf"),
                font_size: 36.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            ..default()
        }),MyText
    ));

    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                color:Color::WHITE,
                font: asset_server.load("pop_warner.ttf"),
                font_size: 24.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top:Val::Px(40.0),
            ..default()
        }), DispText
        ));


    let myscene = asset_server.load("A.gltf#Scene0");
    println!("{:?}",myscene);
    commands.spawn((SceneBundle {
        scene: myscene,
        transform: Transform::from_xyz(3.2, 0.05, 0.1),
        ..default()
        }, TextEnt3D{spacing:0.0},)
    );
}

fn listen_kb_events(
    mut events: EventReader<KeyboardInput>,
    mut edit_text: Query<(Entity, &mut Text), With<MyText>>,
    mut entered_text: ResMut<EnteredText>,
    mut text_entry: EventWriter<NewTextEntry>,
) { 
    for event in events.read() {
        let (ent,mut text) = edit_text.single_mut();
        match event.key_code {
            Some(KeyCode::Return) => {
                let text_string = &text.sections[0].value;
                if text_string.len()>0 { 
                    entered_text.text = text_string.clone();
                    text_entry.send(NewTextEntry(entered_text.text.clone()));
                }
                text.sections[0].value = format!("");
            }
            Some(KeyCode::Back) => { 
                text.sections[0].value.pop();
                println!("backed");
            }
            _ => continue,
        }
    }
}

fn listen_rx_char_events(
    mut events: EventReader<ReceivedCharacter>,
    mut edit_text: Query<&mut Text, With<MyText>>,
) {
    for event in events.read() {
                if event.char != '\r' {
                    edit_text.single_mut().sections[0].value.push(event.char);
                }
        }
}

fn display_et(
    mut disp_text: Query<(Entity, &mut Text), With<DispText>>,
    mut entered_text: ResMut<EnteredText>
){
    let (ent,mut text) = disp_text.single_mut();
    text.sections[0].value = entered_text.text.clone();
}


fn shift_left(
    mut commands:Commands,
    mut query: Query<&mut Transform, With<TextEnt3D>>, 
    time: Res<Time>,
    entered_text: Res<EnteredText>,
    mut text_entry: EventWriter<NewTextEntry>,
    mut edit_text: Query<(Entity, &mut Text), With<MyText>>,
    mut spawned_entities_3d: Query<Entity,With<TextEnt3D>>,
) {
    let x_translate = 2.0*0.4*time.delta_seconds();

    for mut transform in &mut query {
            
            if transform.translation.x < -3.2-(entered_text.text.len() as f32)*0.5 {
                // let (ent,mut text) = edit_text.single_mut();
                // let text_string = &text.sections[0].value;
                // if text_string.len()>0 { 
                    for entity in spawned_entities_3d.iter(){
                        println!("test entity");
                        commands.entity(entity).despawn_recursive();
                    }
                    text_entry.send(NewTextEntry(entered_text.text.clone()));
    
            }
            else { 
                transform.translation.x -= x_translate ;
            }
    }
}


fn place_text_mesh(
    mut commands: Commands,
    mut entered_text_events: EventReader<NewTextEntry>, 
    mut spawned_entities_3d: Query<Entity,With<TextEnt3D>>,
    asset_server: Res<AssetServer>,
    entered_text: Res<EnteredText>,

){

    for event in entered_text_events.read() { 
    //clear current meshes first
        for entity in spawned_entities_3d.iter(){
            println!("test entity");
            commands.entity(entity).despawn_recursive();
        }
        //store all spawned letter spacing in a vec
        let mut spacing_vec:Vec<f32> = Vec::new(); 
        let et_string = entered_text.text.clone();
        for (i,key) in et_string.chars().enumerate() { 
            let (file_prefix,spacing) = get_file_prefix(key);
            
            let file_handle = format!("{file_prefix}.gltf#Scene0");
            let myscene = asset_server.load(file_handle);
            let spacing_sum:f32 = spacing_vec.iter().sum();
            let text_loc:f32 = 2.2 + spacing_sum;
            if key != ' ' {
                commands.spawn((SceneBundle {
                    scene: myscene,
                    transform: Transform::from_xyz(text_loc, 0.05, 1.0),
                    //visibility:Hidden,
                    ..default()
                    }, TextEnt3D{spacing:i as f32},)
                );
            } 

            spacing_vec.push(spacing);
        }
    }

}

fn get_file_prefix(key: char) -> (String,f32){ 
    let mut file_prefix:String = "".to_string(); 
    let mut spacing:f32 = 0.0; 
    (file_prefix, spacing) = match key {
        '0' => ("0".to_string(), 0.5),
        '1' => ("1".to_string(), 0.3),
        '2' => ("2".to_string(), 0.4),
        '3' => ("3".to_string(), 0.4),
        '4' => ("4".to_string(), 0.5),
        '5' => ("5".to_string(), 0.4),
        '6' => ("6".to_string(), 0.45),
        '7' => ("7".to_string(), 0.4),
        '8' => ("8".to_string(),0.5),
        '9' => ("9".to_string(),0.5),
        'a'|'A' =>("A".to_string(),0.57),
        'b'|'B' =>("B".to_string(),0.5),
        'c'|'C' =>("C".to_string(),0.42),
        'd'|'D' =>("D".to_string(),0.55),
        'e'|'E' =>("E".to_string(),0.45),
        'f'|'F' =>("F".to_string(),0.40),
        'g'|'G' =>("G".to_string(),0.55),
        'h'|'H' =>("H".to_string(),0.55),
        'i'|'I' =>("I".to_string(),0.35),
        'j'|'J' =>("J".to_string(),0.5),
        'k'|'K' =>("K".to_string(),0.5),
        'l'|'L' =>("L".to_string(),0.42),
        'm'|'M' =>("M".to_string(),0.62),
        'n'|'N' =>("N".to_string(),0.58),
        'o'|'O' =>("O".to_string(),0.57),
        'p'|'P' =>("P".to_string(),0.5),
        'q'|'Q' =>("Q".to_string(), 0.6),
        'r'|'R' =>("R".to_string(),0.46),
        's'|'S' =>("S".to_string(), 0.42),
        't'|'T' =>("T".to_string(),0.4),
        'u'|'U' =>("U".to_string(),0.6),
        'v'|'V' =>("V".to_string(),0.55),
        'w'|'W' =>("W".to_string(),0.75),
        'x'|'X' =>("X".to_string(),0.6),
        'y'|'Y' =>("Y".to_string(), 0.45),
        'z'|'Z' =>("Z".to_string(), 0.45),
        '/' =>("Backslash".to_string(),0.4),
        ':' =>("Colon".to_string(),0.3),
        ';' =>("SemiColon".to_string(),0.3),
        ',' =>("Comma".to_string(),0.3),
        '$' =>("Dolla".to_string(),0.5),
        '=' =>("Equal".to_string(),0.4),
        '!' =>("ExM".to_string(),0.35),
        '#' =>("Hash".to_string(),0.5),
        '(' =>("LeftPah".to_string(),0.2),
        '-' =>("Minus".to_string(),0.2),
        '%' =>("Percent".to_string(),0.55),
        '.' =>("Period".to_string(),0.2),
        '+' =>("Plus".to_string(),0.35),
        '?' =>("QM".to_string(),0.4),
        '"' =>("Quote".to_string(),0.2),
        ')' =>("RightPah".to_string(),0.3),
        '*' =>("Star".to_string(),0.2),
        '_'|' ' =>("Underscore".to_string(),0.35),
        '^' =>("UpHat".to_string(),0.3),
        '\'' =>("Apo".to_string(),0.2),
        _ =>("QM".to_string(),0.2),
    };
    return (file_prefix,spacing);
}




#[derive(Resource)]
pub struct EnteredText{
    pub text:String,
}

#[derive(Component)]
struct TextEnt3D {
    spacing:f32,
}

#[derive(Component)]
struct DispText;

#[derive(Component)]
struct MyText;

#[derive(Event,Debug)]
pub struct NewTextEntry(pub String);