use bevy::{
    core_pipeline::prepass::DepthPrepass,
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_resource::{
            TextureDescriptor, TextureFormat, TextureUsages, TextureDimension, Extent3d,
        },
        view::ViewDepthTexture,
        RenderApp,
    },
};

use crate::post_processing::EffectsCamera;
use bevy_magic_fx::magicfx_material::MagicFxMaterial;

/// Plugin that extracts depth buffer from Effects camera and applies it to MagicFx materials
pub struct CustomDepthPlugin;

impl Plugin for CustomDepthPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<EffectsDepthExtractor>::default())
            .init_resource::<EffectsDepthTexture>()
            .add_systems(Update, (
                setup_effects_camera_depth,
                extract_effects_depth_texture,
                update_magic_fx_depth_textures,
            ).chain());
    }
}

/// Component to mark cameras that should have their depth extracted
#[derive(Component, Clone, ExtractComponent)]
pub struct EffectsDepthExtractor;

/// Resource to hold the depth texture from Effects camera
#[derive(Resource, Default)]
pub struct EffectsDepthTexture {
    pub depth_texture: Option<Handle<Image>>,
}

/// System to set up Effects camera with proper depth configuration
pub fn setup_effects_camera_depth(
    mut commands: Commands,
    effects_camera_query: Query<Entity, (With<EffectsCamera>, Without<EffectsDepthExtractor>)>,
) {
    for entity in effects_camera_query.iter() {
        commands.entity(entity).insert((
            EffectsDepthExtractor,
            DepthPrepass,
        ));
    }
}

/// System to extract depth texture from Effects camera using ViewDepthTexture
fn extract_effects_depth_texture(
    mut images: ResMut<Assets<Image>>,
    mut effects_depth: ResMut<EffectsDepthTexture>,
    effects_camera_query: Query<&ViewDepthTexture, With<EffectsCamera>>,
) {
    let Ok(view_depth_texture) = effects_camera_query. single() else {
    	warn!("no view depth tex ");
        return;
    };


    let texture = & view_depth_texture.texture ; 

    // Create an Image asset that wraps the existing depth texture view
    // This allows us to bind the actual depth buffer to materials
    let size = view_depth_texture.texture.size();
    
    let   depth_image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("effects_depth_texture_wrapper"),
            size: Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: size.depth_or_array_layers,
            },
            mip_level_count: view_depth_texture.texture.mip_level_count(),
            sample_count: view_depth_texture.texture.sample_count(),
            dimension: TextureDimension::D2,
            format: view_depth_texture.texture.format(),
            usage: TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        },
        // We don't need to populate data since we're wrapping the existing GPU texture
        data: Some(Vec::new()),

        ..default() 
    };

    // Store the GPU texture directly in the Image asset
    // This is a bit of a hack but allows us to bind the depth buffer directly
    let handle = images.add(depth_image);
    effects_depth.depth_texture = Some(handle);
    
    info!("Using effects camera depth texture directly: {}x{}", size.width, size.height);
}

/// System that applies the depth texture to all MagicFx materials
fn update_magic_fx_depth_textures(
    effects_depth: Res<EffectsDepthTexture>,
    mut magic_fx_materials: ResMut<Assets<MagicFxMaterial>>,
) {
    // Update all MagicFx materials with the depth texture
    if let Some(depth_texture) = &effects_depth.depth_texture {
        for (_, material) in magic_fx_materials.iter_mut() {
            material.extension.custom_depth_texture = Some(depth_texture.clone());
        }
    }
}