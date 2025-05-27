 



    use bevy::prelude::*;
use bevy_editor_pls_core::editor_window::{EditorWindow, EditorWindowContext};
use bevy_inspector_egui::egui::{self, RichText};

 
#[derive(Component)]
 pub struct Sun ;
 

 
pub struct LightingWindowState {
   ambient_light_illumination: f32,

   enable_dynamic_shadows: bool 
   // scene_save_result: Option<Result<(), Box<dyn std::error::Error + Send + Sync>>>,
}

impl Default for LightingWindowState {



	fn default() -> Self { 
		Self{ 
            ambient_light_illumination: 600.0 ,
            enable_dynamic_shadows: true 

        } 
	}
}

pub struct LightingWindow;

impl EditorWindow for LightingWindow {
    type State = LightingWindowState;
    const NAME: &'static str = "Lighting";

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let state = cx.state_mut::<LightingWindow>().unwrap();

       let mut ambient_lighting = world.resource_mut::<AmbientLight>() ;



        ui.horizontal(|ui| {
           
            ui.add(
                    egui::Slider::new(&mut state.ambient_light_illumination, 0.0..=10000.0)
                        .text("Ambient Light Illumination"),
                );

        });

        ambient_lighting.brightness = state.ambient_light_illumination.clone();

    
    }


     fn app_setup(app: &mut App) {
        app

        //.init_resource::<PreviouslyActiveCameras>();

         .add_systems(Startup, initialize_ambient_lighting) 

          ;
    }

}


fn initialize_ambient_lighting(
	mut commands:Commands

	){ 

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 600.0 ,
        ..default() 
    });
 
} 

 