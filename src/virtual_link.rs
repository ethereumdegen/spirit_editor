use bevy::prelude::*;

use bevy::utils::HashMap;

use bevy::{color::palettes::css::* };
use spirit_edit_core::zones::zone_file::CustomPropsComponent;



pub fn virtual_links_plugin(  app: &mut App) {


	app
	.register_type::<VirtualLinkComponent>()
	.init_resource::<UniqueNameRegistry>()
	.add_systems(Update, 
				(	
					add_unique_name_components, 
					add_virtual_link_from_custom_props, 
					add_virtual_link_from_unique_name, 
					render_gizmo_lines
				).chain()

		  )


	;


}

 #[derive(Resource,Default)]
 pub struct UniqueNameRegistry  (pub HashMap<String,Entity>);


#[derive(Component,Debug,Clone )]
pub struct UniqueNameComponent  (pub String); 

#[derive(Component,Debug,Clone,Reflect )]
#[reflect(Component)]
pub struct VirtualLinkComponent {


	pub target_entity: Entity, 

	//link type ? 
}

fn render_gizmo_lines (

	global_xform_query: Query<&GlobalTransform>,

	virtual_link_query: Query<(Entity, &VirtualLinkComponent)>, 

    mut gizmos: Gizmos,
){

	for (link_source_entity, virtual_link ) in virtual_link_query.iter(){
 

		let Some(source_xform) = global_xform_query.get( link_source_entity ).ok() else {continue};

		let Some(target_xform) = global_xform_query.get( virtual_link.target_entity ).ok() else {continue};

 
		 gizmos.arrow(source_xform.translation(), target_xform.translation(), YELLOW);

 

	}
 



}

fn add_unique_name_components (

	mut commands : Commands , 

	custom_props_query: Query<(Entity,&CustomPropsComponent), Changed<CustomPropsComponent>>,

	mut unique_name_registry: ResMut<UniqueNameRegistry>
){


	for (  entity, custom_props_comp ) in custom_props_query.iter(){



		if let Some(unique_name) =  custom_props_comp.props.get("unique_name") {


			unique_name_registry.0.insert( unique_name.to_string().clone(), entity.clone()) ;


			if let Some(mut cmd) = commands.get_entity( entity ){

				cmd.try_insert( 
					UniqueNameComponent( unique_name.to_string().clone() )
				 );

			}

		}

 

	}



}





fn add_virtual_link_from_custom_props(
	mut commands:Commands, 

	custom_props_query: Query<(Entity,&CustomPropsComponent), Changed<CustomPropsComponent>>,

//	unique_name_query: Query<(Entity, &UniqueNameComponent)>

 

){


	for (  entity, custom_props_comp ) in custom_props_query.iter(){ 
				commands.entity(entity).queue(RefreshVirtualLink);


	}
 


}



fn add_virtual_link_from_unique_name(
	mut commands:Commands, 

	custom_props_query: Query<Entity, With<CustomPropsComponent> >,

	unique_name_query: Query<(Entity, &UniqueNameComponent), Changed<UniqueNameComponent>>

 

){




for (  _unique_name_entity, unique_name_comp ) in unique_name_query.iter(){ 


	for custom_prop_entity in &custom_props_query {


		commands.entity(custom_prop_entity).queue( RefreshVirtualLink );

	}
 
	 
}

 


}


struct RefreshVirtualLink; 

impl EntityCommand for RefreshVirtualLink  { 


	fn apply(self, script_entity: Entity, world: &mut World) { 


		let mut unique_name_query = world.query::<(Entity, &UniqueNameComponent)>();


	   if let Some(mut cmd) = world.commands().get_entity( script_entity ){

			cmd.remove::<VirtualLinkComponent>();

		}


		let Some(custom_props_comp) = world.get::<CustomPropsComponent>(script_entity) else {return};



		if let Some(target_unique_name) =  custom_props_comp.props.get("target_unique_name") {
				//use registry? 
			for (target_entity, unique_name_comp) in unique_name_query.iter(  world  ){

				if unique_name_comp.0 == target_unique_name.to_string() {

 
				    if let Some(mut cmd) = world.commands().get_entity( script_entity ){

						cmd.try_insert( 
							VirtualLinkComponent {
						        target_entity,
						    } 
						 );

					}


					break;
				}
			} 


		}


	}
}