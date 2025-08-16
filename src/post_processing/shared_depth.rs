

use bevy::{
    core_pipeline::{
        core_3d::graph::{Core3d, Node3d},
        core_3d::graph::input::VIEW_ENTITY,
    },
    prelude::*,
    render::{
        render_graph::{
            Node, NodeRunError, RenderGraph, RenderGraphContext, 
            SlotInfo, SlotType
        },
        render_phase::{RenderPhase, Opaque3d, AlphaMask3d, Transparent3d},
        render_resource::*,
        renderer::{RenderContext, RenderDevice},
        view::{ExtractedCamera, ViewTarget, ViewDepthTexture},
        texture::{TextureCache, TextureDescriptor, TextureFormat, TextureUsages, TextureDimension},
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        RenderApp, RenderSet, Render,
    },
};

// Plugin to set up our custom render graph
pub struct SharedDepthRenderPlugin;

impl Plugin for SharedDepthRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<SharedDepthCamera>::default())
            .add_systems(
                PostUpdate,
                setup_shared_depth_cameras.run_if(any_with_component::<SharedDepthCamera>),
            );
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        
        render_app
            .init_resource::<SharedDepthTextures>()
            .add_systems(
                Render,
                (
                    prepare_shared_depth_textures.in_set(RenderSet::Prepare),
                    extract_shared_depth_textures.in_set(RenderSet::PrepareBindGroups),
                )
            );

        // Get the 3D render graph and modify it
        let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        
        // Add our custom nodes to the 3D graph
        render_graph.add_node(
            Core3d,
            SHARED_DEPTH_MAIN_PASS,
            SharedDepthMainPassNode::new(),
        );
        
        render_graph.add_node(
            Core3d,
            SHARED_DEPTH_OVERLAY_PASS,
            SharedDepthOverlayPassNode::new(),
        );

        // Set up dependencies
        render_graph.add_node_edge(Core3d, Node3d::Prepass, SHARED_DEPTH_MAIN_PASS);
        render_graph.add_node_edge(Core3d, SHARED_DEPTH_MAIN_PASS, SHARED_DEPTH_OVERLAY_PASS);
        render_graph.add_node_edge(Core3d, SHARED_DEPTH_OVERLAY_PASS, Node3d::MainTransparentPass3d);
    }
}

// Component to mark cameras that participate in shared depth rendering
#[derive(Component, Clone, ExtractComponent)]
pub struct SharedDepthCamera {
    pub is_main: bool, // true for RenderLayer0, false for RenderLayer2
}

// Resource to hold shared depth textures
#[derive(Resource, Default)]
pub struct SharedDepthTextures {
    pub main_depth: Option<TextureView>,
    pub main_depth_texture: Option<Texture>,
}

// Node labels for our custom render graph nodes
pub const SHARED_DEPTH_MAIN_PASS: &str = "shared_depth_main_pass";
pub const SHARED_DEPTH_OVERLAY_PASS: &str = "shared_depth_overlay_pass";

// Custom render node for the main pass (RenderLayer0)
pub struct SharedDepthMainPassNode {
    main_query: QueryState<(
        &'static ExtractedCamera,
        &'static RenderPhase<Opaque3d>,
        &'static RenderPhase<AlphaMask3d>,
        &'static ViewTarget,
        &'static ViewDepthTexture,
        &'static SharedDepthCamera,
    )>,
}

impl SharedDepthMainPassNode {
    pub fn new() -> Self {
        Self {
            main_query: QueryState::new(),
        }
    }
}

impl Node for SharedDepthMainPassNode {
    fn input(&self) -> Vec<SlotInfo> {
        vec![SlotInfo::new(VIEW_ENTITY, SlotType::Entity)]
    }

    fn update(&mut self, world: &mut World) {
        self.main_query.update_archetypes(world);
    }

    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let view_entity = graph.get_input_entity(VIEW_ENTITY)?;
        
        if let Ok((
            camera,
            opaque_phase,
            alpha_mask_phase,
            target,
            depth_texture,
            shared_depth_camera,
        )) = self.main_query.get_manual(world, view_entity) {
            
            // Only process main cameras (RenderLayer0)
            if !shared_depth_camera.is_main {
                return Ok(());
            }

            // Store the depth texture for overlay pass to use
            let mut shared_textures = world.resource_mut::<SharedDepthTextures>();
            shared_textures.main_depth = Some(depth_texture.view.clone());
            shared_textures.main_depth_texture = Some(depth_texture.texture.clone());

            // Render the main pass normally
            let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
                label: Some("shared_depth_main_pass"),
                color_attachments: &[Some(target.get_color_attachment())],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &depth_texture.view,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(0.0),
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Set the camera and render
            if let Some(viewport) = camera.viewport.as_ref() {
                render_pass.set_camera_constants(camera);
            }

            // Render opaque phase
            opaque_phase.render(&mut render_pass, world, view_entity);
            
            // Render alpha mask phase  
            alpha_mask_phase.render(&mut render_pass, world, view_entity);
        }

        Ok(())
    }
}

// Custom render node for the overlay pass (RenderLayer2) 
pub struct SharedDepthOverlayPassNode {
    overlay_query: QueryState<(
        &'static ExtractedCamera,
        &'static RenderPhase<Transparent3d>,
        &'static ViewTarget,
        &'static ViewDepthTexture,
        &'static SharedDepthCamera,
    )>,
}

impl SharedDepthOverlayPassNode {
    pub fn new() -> Self {
        Self {
            overlay_query: QueryState::new(),
        }
    }
}

impl Node for SharedDepthOverlayPassNode {
    fn input(&self) -> Vec<SlotInfo> {
        vec![SlotInfo::new(VIEW_ENTITY, SlotType::Entity)]
    }

    fn update(&mut self, world: &mut World) {
        self.overlay_query.update_archetypes(world);
    }

    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let view_entity = graph.get_input_entity(VIEW_ENTITY)?;
        
        if let Ok((
            camera,
            transparent_phase,
            target,
            depth_texture,
            shared_depth_camera,
        )) = self.overlay_query.get_manual(world, view_entity) {
            
            // Only process overlay cameras (RenderLayer2)
            if shared_depth_camera.is_main {
                return Ok(());
            }

            // Get the shared depth texture from the main pass
            let shared_textures = world.resource::<SharedDepthTextures>();
            let main_depth_view = shared_textures.main_depth.as_ref()
                .ok_or(NodeRunError::MissingInput("main_depth_texture".into()))?;

            // Render overlay pass using the main camera's depth for testing
            let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
                label: Some("shared_depth_overlay_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: target.main_texture_view(),
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Load, // Don't clear, we want to render on top
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: main_depth_view, // Use main camera's depth!
                    depth_ops: Some(Operations {
                        load: LoadOp::Load, // Keep existing depth
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Set camera and render overlay objects
            if let Some(viewport) = camera.viewport.as_ref() {
                render_pass.set_camera_constants(camera);
            }

            // Render transparent phase (your RenderLayer2 objects)
            transparent_phase.render(&mut render_pass, world, view_entity);
        }

        Ok(())
    }
}

// System to prepare shared depth textures
fn prepare_shared_depth_textures(
    mut shared_textures: ResMut<SharedDepthTextures>,
    render_device: Res<RenderDevice>,
    mut texture_cache: ResMut<TextureCache>,
    cameras: Query<(&ExtractedCamera, &SharedDepthCamera)>,
) {
    // Find main camera dimensions to create appropriately sized textures
    for (camera, shared_depth) in cameras.iter() {
        if shared_depth.is_main {
            if let Some(size) = camera.physical_target_size {
                // Ensure we have a depth texture of the right size
                let depth_descriptor = TextureDescriptor {
                    label: Some("shared_main_depth"),
                    size: Extent3d {
                        width: size.x,
                        height: size.y,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::Depth32Float,
                    usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                };

                let texture = texture_cache.get(&render_device, depth_descriptor);
                shared_textures.main_depth_texture = Some(texture.texture.clone());
                shared_textures.main_depth = Some(texture.default_view.clone());
                break;
            }
        }
    }
}

// System to extract shared depth texture info
fn extract_shared_depth_textures(
    mut commands: Commands,
    shared_textures: Res<SharedDepthTextures>,
) {
    // This system can be used to bind the shared depth texture 
    // to materials that need it (like your RenderLayer2 materials)
    
    // You would extract the texture handle and make it available
    // to your custom materials here
}

// System to set up cameras with shared depth component
fn setup_shared_depth_cameras(
    mut commands: Commands,
    cameras: Query<(Entity, &RenderLayers), (With<Camera3d>, Without<SharedDepthCamera>)>,
) {
    for (entity, render_layers) in cameras.iter() {
        let is_main = render_layers.intersects(&RenderLayers::layer(0));
        
        commands.entity(entity).insert(SharedDepthCamera { is_main });
    }
}