

use bevy::prelude::*;

use doodads::DoodadPlugin;
use zones::{zone_file::{CustomProp, CustomPropsComponent},  ZoneEvent, ZoneResource};


pub mod doodads;
pub mod zones;
pub mod placement;


pub struct SpiritEditCorePlugin {}
impl Plugin for SpiritEditCorePlugin {
    fn build(&self, app: &mut App) {
        //put this inside of zone plugin ?
         app
           	

           	 .add_event::<placement::PlacementEvent>()
           	 .add_event::<doodads::picking::SelectDoodadEvent>()
            .init_resource::<placement::PlacementResource>()

            .init_resource::<placement::PlacementToolsState>()


            .add_event::<ZoneEvent>()
           
            .register_type::<CustomPropsComponent>() //reflect
              .register_type::<CustomProp>() //reflect
           
            .init_resource::<ZoneResource>() 

           
         
            
            .add_systems(Update, zones::handle_zone_events)


            .add_plugins(DoodadPlugin {})
              
             


            ;
    }
}


