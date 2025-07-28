

pub mod rendering; 

/*

use crate::vfx::visual_effects::VfxEffectEvents;
use crate::vfx::visual_effects::VfxClipTypeOrStem;
use crate::render::rendering::GraphicsRenderLayer;

use crate::game_time::GameTime;
use std::time::Duration;
use crate::AppState;
use crate::player_control::camera::IngameCamera;*/

// pub mod radial_gradient_material;
// use radial_gradient_material::{RadialGradientMaterial, radial_gradient_material_plugin};
use bevy_editor_pls_default_windows::cameras::EditorCamera;
use crate::post_processing::rendering::GraphicsRenderLayer;
use std::time::Duration;
use bevy::{
    core_pipeline::{
        core_3d::graph::{Core3d, Node3d},
        fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    ecs::query::QueryItem,
    prelude::*,
    render::{
        camera::RenderTarget,
        extract_component::{
            ComponentUniforms, DynamicUniformIndex, ExtractComponent, ExtractComponentPlugin,
        },
        render_graph::{
            NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            binding_types::{sampler, texture_2d},
            *,
        },
        render_asset::RenderAssets,
        texture::GpuImage,
        renderer::{RenderContext, RenderDevice},
        view::{RenderLayers, ViewTarget},
        RenderApp,
    },
};

const SHADER_ASSET_PATH: &str = "shaders/post_processing.wgsl";

pub(crate) fn post_processing_plugin(app: &mut App) {
    app.add_plugins(PostProcessPlugin);


 //   radial_gradient_material_plugin(app);
    app.add_systems(Startup, setup_effects_camera)
       
        .add_systems(Update, (

           sync_effects_camera_with_main_camera,
            update_effects_texture,

          
           
        ).chain() );
}

struct PostProcessPlugin;

impl Plugin for PostProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<PostProcessSettings>::default())
            .init_resource::<EffectsTextureHandle>();

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<PostProcessNode>>(
                Core3d,
                PostProcessLabel,
            )
            .add_render_graph_edges(
                Core3d,
                (
                    Node3d::Tonemapping,
                    PostProcessLabel,
                    Node3d::EndMainPassPostProcessing,
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<PostProcessPipeline>();
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct PostProcessLabel;

#[derive(Default)]
struct PostProcessNode;

impl ViewNode for PostProcessNode {
    type ViewQuery = (&'static ViewTarget, &'static PostProcessSettings);

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, settings): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let post_process_pipeline = world.resource::<PostProcessPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let gpu_images = world.resource::<RenderAssets<GpuImage>>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        let post_process = view_target.post_process_write();

        // Get the effects texture from GPU resources, fallback to default if not available
        let effects_texture_view = if let Some(gpu_image) = gpu_images.get(&settings.effects_texture) {
            &gpu_image.texture_view
        } else {
            &post_process_pipeline.default_effects_texture
        };

        let bind_group = render_context.render_device().create_bind_group(
            "post_process_bind_group",
            &post_process_pipeline.layout,
            &BindGroupEntries::sequential((
                post_process.source,
                &post_process_pipeline.sampler,
                effects_texture_view,
                &post_process_pipeline.effects_sampler,
            )),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("post_process_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

#[derive(Resource)]
struct PostProcessPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    effects_sampler: Sampler,
    default_effects_texture: TextureView,
    pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for PostProcessPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "post_process_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                ),
            ),
        );

        let sampler = render_device.create_sampler(&SamplerDescriptor::default());
        let effects_sampler = render_device.create_sampler(&SamplerDescriptor::default());

        // Create a default 1x1 black texture for effects when no specific effects texture is provided
        let default_effects_texture = render_device.create_texture(&TextureDescriptor {
            label: Some("default_effects_texture"),
            size: Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let default_effects_texture_view = default_effects_texture.create_view(&TextureViewDescriptor::default());

        let shader = world.load_asset(SHADER_ASSET_PATH);

        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("post_process_pipeline".into()),
                layout: vec![layout.clone()],
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader,
                    shader_defs: vec![],
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::Rgba16Float, // Match HDR camera format
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
                zero_initialize_workgroup_memory: false,
            });

        Self {
            layout,
            sampler,
            effects_sampler,
            default_effects_texture: default_effects_texture_view,
            pipeline_id,
        }
    }
}

#[derive(Component, Default, Clone, ExtractComponent)]
pub struct PostProcessSettings {
    pub effects_texture: Handle<Image>,
}

 

// Resource to hold the effects texture handle
#[derive(Resource, Default)]
pub struct EffectsTextureHandle {
    pub handle: Handle<Image>,
}

// Marker component for the effects camera
#[derive(Component)]
pub struct EffectsCamera;

// Marker component for objects that should only render to the effects layer
//#[derive(Component)]
//pub struct EffectsObject;

// System to set up the effects camera that renders to a texture
fn setup_effects_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut effects_texture_handle: ResMut<EffectsTextureHandle>,
) {
    // Create the render texture for effects
    let size = Extent3d {
        width: 1024,
        height: 1024,
        depth_or_array_layers: 1,
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("effects_render_texture"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);

    let image_handle = images.add(image);
    effects_texture_handle.handle = image_handle.clone();


    // why does this break doodad rendering ? 

    // Spawn the effects camera that renders only layer 2 to the texture
    commands.spawn((
        EffectsCamera,
        Camera3d::default(),
        Camera {
            target: RenderTarget::Image(image_handle.clone().into()),
            clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 1.0)), // Clear to black
          
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::layer( GraphicsRenderLayer::PostProcessingEffect .into() ), // Only render layer 2
    ));


 
}

// System to update the effects texture on cameras with PostProcessSettings
fn update_effects_texture(
    effects_texture_handle: Res<EffectsTextureHandle>,
    mut post_process_query: Query<&mut PostProcessSettings>,
) {
    for mut settings in post_process_query.iter_mut() {
        settings.effects_texture = effects_texture_handle.handle.clone();
    }
}

// Helper function to spawn an effects object (like a sphere) that only renders to the effects layer
/* pub fn spawn_effects_sphere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let position = Vec3::new(62.0, 18.0, 49.0);
    let radius = 2.0; // Make it bigger to see the effect better 

    commands.spawn((
        Name::new("Standard Effects Sphere"),
        Mesh3d(meshes.add(Sphere::new(radius))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            unlit: true, // Use unlit so it renders as pure color
            ..default()
        })),
        Transform::from_translation(position),
        RenderLayers::layer(EFFECTS_LAYER.into()), // Only render on effects layer
    ));
}

*/
 

/*

pub fn spawn_test_abberation_sphere(world: &mut World) {
    let position = Vec3::new(62.0, 18.0, 48.0);
    let radius = 1.0;
    let duration = Duration::from_secs_f32( 90.0 ) ; 

    world.commands().queue(

        SpawnAbberationSphere {
            position,
            radius,
            duration, 
        } 


 
 
    );
    
}


// this works !!! 
pub fn spawn_test_magic_fx(world: &mut World){

    let position = Vec3::new(62.0, 18.0, 49.0) .into();
    let vfx_clip_type = VfxClipTypeOrStem::Stem(
        "light_sphere_test.magicfx" .into()

        );
    let render_layer = GraphicsRenderLayer::PostProcessingEffect .into (); 
   //let render_layer = GraphicsRenderLayer::Standard3d .into (); 


    world.commands().send_event(


            VfxEffectEvents::RenderMagicVfx {
                vfx_clip_type,
                render_layer,
                position 

            }

        );
}


*/

// System to sync the effects camera transform with the main camera
fn sync_effects_camera_with_main_camera(
    main_camera_query: Query<(&Transform, &Projection), (With<EditorCamera>, Without<EffectsCamera>)>,
    mut effects_camera_query: Query<(&mut Transform, &mut Projection), (With<EffectsCamera>, Without<EditorCamera>)>,
) {
    let Ok((main_camera_transform, main_projection)) = main_camera_query.single() else {
        return; // No main camera found
    };
    
    let Ok((mut effects_camera_transform, mut effects_projection)) = effects_camera_query.single_mut() else {
        return; // No effects camera found
    };
    
    // Copy the main camera's transform to the effects camera
    *effects_camera_transform = *main_camera_transform;
    
    // Also copy the projection to ensure FOV and near/far planes match
    *effects_projection = main_projection.clone();
}


 