 

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy_mesh_terrain::{TerrainMeshPlugin, terrain::{TerrainConfig, TerrainData, TerrainViewer}, edit::{EditTerrainEvent}};

use bevy_mesh_terrain::edit::EditingTool;


use bevy_mod_raycast::prelude::*;


fn main() {
    App::new()
        
         
          .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window { 
                    present_mode:  bevy::window::PresentMode::AutoNoVsync, //improves latency

                    title: "Terrain Edit".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            })
        )

        .add_plugins(DefaultRaycastingPlugin)
         
        .add_plugins( TerrainMeshPlugin::default() )
          
        .add_systems(Startup, setup) 

        //move to brushes and tools lib 
        .add_systems(Update, update_brush_paint )
        
        
        //move to camera lib 
        .add_systems(Update, update_camera_look ) 
        .add_systems(Update, update_camera_move ) 
        
     
        
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    
   
    asset_server: Res<AssetServer> 
) {
    
    
    let array_texture: Handle<Image> = asset_server.load("terrain/textures/array_texture_lg_2.png");
    let height_map: Handle<Image> = asset_server.load("terrain/source/height.png"); 
    let splat_texture: Handle<Image> = asset_server.load("terrain/textures/splat_texture.png");
     
    
    
   
    commands.spawn(VisibilityBundle::default() ) 
    .insert( TransformBundle::default() )
    .insert(
        TerrainConfig::default()
        .set_render_distance( 1500.0 )
        )
    .insert(
        TerrainData::default()
        .add_height_map_image(   height_map  ) 
        .add_array_texture_image(array_texture, 8) 
        .add_splat_texture_image( splat_texture )
    ); 
    
     
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight::default(),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 800.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(20.0, 162.5, 20.0).looking_at(Vec3::new(900.0,0.0,900.0), Vec3::Y),
        ..default()
    })
    .insert(TerrainViewer::default());
    
   
}


 

 
fn update_brush_paint( 
    mouse_input:  Res< Input<MouseButton> > , //detect mouse click 
        
    cursor_ray: Res<CursorRay>, 
    mut raycast: Raycast,    
      
    mut event_writer: EventWriter<EditTerrainEvent>,
){
     
     
     if !mouse_input.pressed(MouseButton::Left) {
        return;
    }
    
    //if tool is paintbrush ... (conditional check)
     
     //make me dynamic or whatever 
    let tool = EditingTool::SetHeightMap(25,25.0);
    
   
    if let Some(cursor_ray) = **cursor_ray {
       
      
      
      
        if let Some((intersection_entity,intersection_data)) = raycast.cast_ray(cursor_ray, &default() ).first(){
            
                       
            let hit_point = intersection_data.position();
                         
             
             //offset this by the world psn offset of the entity !? would need to query its transform ?  for now assume 0 offset.
            let hit_coordinates = Vec2::new(hit_point.x, hit_point.z);
            
            //use an event to pass the entity and hit coords to the terrain plugin so it can edit stuff there 
          
            event_writer.send(EditTerrainEvent {
                entity: intersection_entity.clone(), 
                tool, 
                coordinates:hit_coordinates
            });            
             
            
        } 
        
    }
    
     
    
}
 
 
fn update_camera_look(
    mut event_reader:   EventReader<MouseMotion>  ,
    mouse_input:  Res< Input<MouseButton> > ,
    mut query: Query<(&mut Transform, &Camera3d)>,
    
    
){
    const MOUSE_SENSITIVITY: f32 = 2.0;
     
     if !mouse_input.pressed(MouseButton::Right) {
        return;
    }
    
      
      // Accumulate mouse delta
    let mut delta: Vec2 = Vec2::ZERO;
    for event in event_reader.iter() {
        delta += event.delta;
    }

    // Apply to each camera with the CameraTag
    for (mut transform, _) in query.iter_mut() {
       // let rotation = transform.rotation;
      
        let (mut yaw, mut pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
       
        yaw -= delta.x / 180.0   * MOUSE_SENSITIVITY  ;
        pitch -= delta.y / 180.0   * MOUSE_SENSITIVITY;
        pitch = pitch .clamp(-std::f32::consts::PI / 2.0, std::f32::consts::PI / 2.0) ;
   
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
       
    }
    
}


fn update_camera_move(
   
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Camera3d)>,
    
    
){
      const MOVE_SPEED: f32 = 10.0; // You can adjust this value as needed
     
     
     
  
    // Apply to each camera with the CameraTag
    for (mut transform, _) in query.iter_mut() {
       
      
           // Move the camera forward if W is pressed
        if keyboard_input.pressed(KeyCode::W) {
            let forward = transform.forward();
            transform.translation += forward * MOVE_SPEED;
        }
         
          if keyboard_input.pressed(KeyCode::S) {
            let forward = transform.forward() ;
            transform.translation -= forward * MOVE_SPEED;
        }
         
        
       
    }
    
}
