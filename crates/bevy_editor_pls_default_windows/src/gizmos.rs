 
use bevy_editor_pls_core::Editor;
use transform_gizmo_bevy::{config::TransformPivotPoint, prelude::*};
use bevy::{
    ecs::query::QueryFilter,
    prelude::*,
    render::{camera::CameraProjection, view::RenderLayers},
};

use bevy_editor_pls_core::editor_window::{EditorWindow, EditorWindowContext};
use bevy_inspector_egui::{bevy_inspector::hierarchy::SelectedEntities, egui};
//use egui_gizmo::GizmoMode;

use transform_gizmo_bevy::{Gizmo,GizmoOrientation,GizmoMode};


use crate::{
    cameras::{ActiveEditorCamera, CameraWindow, EditorCamera, EDITOR_RENDER_LAYER},
    hierarchy::HierarchyWindow,
};


/*
pub struct GizmoPlugin {}
impl Plugin for GizmoPlugin {
    fn build(&self, app: &mut App) {
        //put this inside of zone plugin ?
         app

            
           .add_systems(Update, update_gizmo_components)
         
            
            ;
    }
}

*/



pub fn update_gizmo_options (

    mut gizmo_options: ResMut<GizmoOptions>
){

     gizmo_options.gizmo_orientation = GizmoOrientation::Local; 
   // gizmo_options.pivot_point = TransformPivotPoint::



}


pub struct GizmoState {
    pub camera_gizmo_active: bool,
    pub gizmo_mode: EnumSet<GizmoMode>,
}

impl Default for GizmoState {
    fn default() -> Self {
        Self {
            camera_gizmo_active: true,
            gizmo_mode: GizmoMode::all(),
        }
    }
}

pub struct GizmoWindow;

impl EditorWindow for GizmoWindow {
    type State = GizmoState;

    const NAME: &'static str = "Gizmos";

    fn ui(_world: &mut World, _cx: EditorWindowContext, ui: &mut egui::Ui) {
        ui.label("Gizmos can currently not be configured");
    }

    fn viewport_toolbar_ui(world: &mut World, cx: EditorWindowContext, ui: &mut egui::Ui) {
        let gizmo_state = cx.state::<GizmoWindow>().unwrap();

        if gizmo_state.camera_gizmo_active {
            if let (Some(hierarchy_state), Some(_camera_state)) =
                (cx.state::<HierarchyWindow>(), cx.state::<CameraWindow>())
            {
               apply_gizmo_component( world, &hierarchy_state.selected, gizmo_state.gizmo_mode );
             //   draw_gizmo(ui, world, &hierarchy_state.selected, gizmo_state.gizmo_mode);
            }
        }
    }

    fn app_setup(app: &mut App) {
        let mut materials = app.world_mut().resource_mut::<Assets<StandardMaterial>>();
        let material_light = materials.add(StandardMaterial {
            base_color: Color::rgba_u8(222, 208, 103, 255),
            unlit: true,
            fog_enabled: false,
            alpha_mode: AlphaMode::Add,
            ..default()
        });
        let material_camera = materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 1.0, 1.0),
            unlit: true,
            fog_enabled: false,
            alpha_mode: AlphaMode::Multiply,
            ..default()
        });

        let mut meshes = app.world_mut().resource_mut::<Assets<Mesh>>();
        let sphere = meshes.add(Sphere { radius: 0.3 });

        app.world_mut().insert_resource(GizmoMarkerConfig {
            point_light_mesh: sphere.clone(),
            point_light_material: material_light.clone(),
            directional_light_mesh: sphere.clone(),
            directional_light_material: material_light,
            camera_mesh: sphere,
            camera_material: material_camera,
        });

        app.add_systems(PostUpdate, add_gizmo_markers);
    }
}

#[derive(Resource)]
struct GizmoMarkerConfig {
    point_light_mesh: Handle<Mesh>,
    point_light_material: Handle<StandardMaterial>,
    directional_light_mesh: Handle<Mesh>,
    directional_light_material: Handle<StandardMaterial>,
    camera_mesh: Handle<Mesh>,
    camera_material: Handle<StandardMaterial>,
}

#[derive(Component)]
struct HasGizmoMarker;

type GizmoMarkerQuery<'w, 's, T, F = ()> =
    Query<'w, 's, Entity, (With<T>, Without<HasGizmoMarker>, F)>;

fn add_gizmo_markers(
    mut commands: Commands,
    gizmo_marker_meshes: Res<GizmoMarkerConfig>,

    point_lights: GizmoMarkerQuery<PointLight>,
    directional_lights: GizmoMarkerQuery<DirectionalLight>,
    cameras: GizmoMarkerQuery<Camera, Without<EditorCamera>>,
) {
    fn add<T: Component, F: QueryFilter, B: Bundle>(
        commands: &mut Commands,
        query: GizmoMarkerQuery<T, F>,
        name: &'static str,
        f: impl Fn() -> B,
    ) {
        let render_layers = RenderLayers::layer(EDITOR_RENDER_LAYER.into());
        for entity in &query {
            commands
                .entity(entity)
                .insert(HasGizmoMarker)
                .with_children(|commands| {
                    commands.spawn((f(), render_layers.clone(), Name::new(name)));
                });
        }
    }

    add(&mut commands, point_lights, "PointLight Gizmo", || (
         
             Mesh3d(gizmo_marker_meshes.point_light_mesh.clone_weak() ),
            MeshMaterial3d ( gizmo_marker_meshes.point_light_material.clone_weak() ), 
            
        
    ));
    add(
        &mut commands,
        directional_lights,
        "DirectionalLight Gizmo",
        ||  (
            Mesh3d( gizmo_marker_meshes.directional_light_mesh.clone_weak()),
            MeshMaterial3d( gizmo_marker_meshes.directional_light_material.clone_weak() ),
            
        ),
    );

    let render_layers = RenderLayers::layer(EDITOR_RENDER_LAYER.into());


    let add_gizmo_to_cameras = false; // disabled for now !  Was annoying w wireframes. 

    if add_gizmo_to_cameras {

    for entity in &cameras {
            commands
                .entity(entity)
                .insert((
                    HasGizmoMarker,
                    Visibility::Visible,
                   // InheritedVisibility::VISIBLE,
                  //  ViewVisibility::default(),
                ))
                .with_children(|commands| {
                    commands.spawn((
                         
                        Mesh3d(   gizmo_marker_meshes.camera_mesh.clone_weak() ),
                        MeshMaterial3d( gizmo_marker_meshes.camera_material.clone_weak() ),
                       
                        
                        render_layers.clone(),
                        Name::new("Camera Gizmo"),
                    ));
                });
        }
    }
}
 


fn apply_gizmo_component (

     // ui: &mut egui::Ui,
    world: &mut World,
    selected_entities: &SelectedEntities,
    gizmo_mode: EnumSet<GizmoMode>,
    ){




        let mut gizmo_target_query = world.query_filtered::<Entity, With <GizmoTarget>  >(); 
        let   gizmo_target_entities: Vec<_> = gizmo_target_query.iter(world).collect();         

        for target in gizmo_target_entities.iter(){

            if selected_entities.contains(*target) {
                continue;
            }

            if let Some(mut cmd) = world.commands().get_entity(*target){
                cmd.remove::<GizmoTarget>();
            }
              
             
 
        }


          for selected in selected_entities.iter() {
            let Some(transform) = world.get::<Transform>(selected) else {
                continue;
            };

     

            if let Some(mut cmd) = world.commands().get_entity(selected){
                cmd .insert ( GizmoTarget ::default() ) ;
            }
          

        }

}
 