use bevy::prelude::*;

use bevy::utils::HashMap;

use bevy::{color::palettes::css::* };
use bevy_material_tool::material_overrides::MaterialOverrideComponent;
use spirit_edit_core::zones::zone_file::CustomPropsComponent;



pub fn material_overrides_link_plugin(  app: &mut App) {


	app
	.register_type::<MaterialOverrideLinkComponent>()
	 
	.add_systems(Update, 
				(	
					add_material_override_from_custom_props, 
					//add_virtual_link_from_custom_props, 
					//render_gizmo_lines
				).chain()

		  )


	;


}

 
#[derive(Component,Debug,Clone ,Reflect)]
#[reflect(Component)]
pub struct MaterialOverrideLinkComponent  (pub String); 
 

 




fn add_material_override_from_custom_props(
	mut commands:Commands, 

	custom_props_query: Query<(Entity,&CustomPropsComponent), Changed<CustomPropsComponent>>,

	 
  
){


for (  entity, custom_props_comp ) in custom_props_query.iter(){ 
 

		if let Some(material_name) =  custom_props_comp.props.get("material_override") {
				 
				    if let Some(mut cmd) = commands.get_entity( entity ){

						/*cmd.try_insert( 
							MaterialOverrideLinkComponent  (material_name.to_string().clone())
						 );*/

						cmd.try_insert( 
							MaterialOverrideComponent {
								material_override: material_name.to_string().clone()
							}
						 );

					} 
			}
	} 


}