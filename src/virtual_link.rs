use bevy::prelude::*;

use bevy::platform::collections::hash_map::HashMap; 

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
	pub secondary_target_entity: Option<Entity>, 
	 
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


		 if let Some( secondary_target ) = virtual_link.secondary_target_entity {

		 		if let Some(secondary_target_xform) = global_xform_query.get( secondary_target ).ok()  {
		 				 gizmos.arrow(source_xform.translation(), secondary_target_xform.translation(), BLUE);
		 		}

		 }

 

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


			if let Ok(mut cmd) = commands.get_entity( entity ){

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


	// fn apply(self, script_entity: Entity, world: &mut World) { 
		fn apply(self, world_entity: EntityWorldMut ) { 

			 let script_entity = world_entity.id() ; 

		    let  world = world_entity.into_world_mut();

		//let Some(unique_name_comp) = world_entity.get::<UniqueNameComponent>()  else {return}; 
		let mut unique_name_query = world.query::<(Entity, &UniqueNameComponent)>();


		if let Ok(mut cmd) = world.commands().get_entity( script_entity ){

			cmd .remove::<VirtualLinkComponent>();

		}
		
 

		let Some(custom_props_comp) = world.get::<CustomPropsComponent>( script_entity ) else {return};
		let custom_props = custom_props_comp.props.clone(); 



		//-----

		let mut secondary_target_entity = None; 


		if let Some(source_unique_name) =  custom_props.get("source_unique_name") {
				//use registry? 
			for (target_entity, unique_name_comp) in unique_name_query.iter(    world  ){

				if unique_name_comp.0 == source_unique_name.to_string() {
 

					secondary_target_entity = Some(target_entity.clone());

					  

					break;
				}
			} 


		}

		// ------


		if let Some(target_unique_name) =  custom_props.get("target_unique_name") {
				//use registry? 
			for (target_entity, unique_name_comp) in unique_name_query.iter(   world  ){

				if unique_name_comp.0 == target_unique_name.to_string() {

 				
 					/*world_entity.insert(  VirtualLinkComponent {
						        target_entity,
						        secondary_target_entity,
						    }   ) ;*/
				  if let Ok(mut cmd) = world.commands().get_entity( script_entity ){

						cmd.try_insert( 
							VirtualLinkComponent {
						        target_entity,
						        secondary_target_entity,
						    } 
						 );

					} 


					break;
				}
			} 


		}



		


	}
}