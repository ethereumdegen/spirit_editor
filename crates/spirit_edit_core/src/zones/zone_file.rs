use bevy::{prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};

use bevy_clay_tiles::{ clay_tile_block:: ClayTileBlock }; 

#[derive(Serialize, Deserialize)]
pub struct ZoneFile {
    pub entities: Vec<ZoneEntity>, 
}

/*
impl ZoneFile {
    pub fn new(
        entities: Vec<Entity>,
        zone_entity_query: &Query<(&Name, &Transform, Option<&CustomPropsComponent>, Option<&ClayTileBlock>)>,
    ) -> Self {
        let mut zone_entities = Vec::new();

        for entity in entities {
            if let Some(zone_entity) = ZoneEntity::from_entity(entity,  zone_entity_query) {
                zone_entities.push(zone_entity);
            }
        }

        Self {
            entities: zone_entities,
        }
    }
}*/

//reflect makes this show up in the inspector
#[derive(Component, Reflect,Default)]
#[reflect(Component)]
pub struct CustomPropsComponent {
    pub props: CustomPropsMap,
}

impl CustomPropsComponent {
    pub fn set_custom_props_if_empty(&mut self, new_props: &CustomPropsMap){

        for (key,val) in new_props.iter(){

            if self.props.get( key ).is_none(){
                  self.props.insert(key.to_string(), val.clone());
            }
          
        }

    }


    pub fn set_custom_props(&mut self, new_props: &CustomPropsMap){

        for (key,val) in new_props.iter(){


            self.props.insert(key.to_string(), val.clone());
        }

    }
}

pub type CustomPropsMap = HashMap<String, CustomProp>;

#[derive(Serialize, Deserialize, Clone, Debug, Reflect, Default)]
#[reflect(Serialize, Deserialize,Default)]  //need to reflect default or else cant use + in editor 
pub enum CustomProp {
    Vec3(Vec3),
    String(String),
    //StringSpecial(StringSpecial),
    Float(f32),
    Integer(i32), 
    Boolean(bool),
    #[default]
    EmptyProp  
}





//pub struct StringSpecial; 


 



#[derive(Serialize, Deserialize)]
pub struct ZoneEntity {
    pub name: String,

    pub transform: TransformSimple,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_props: Option<CustomPropsMap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clay_tile_block_data: Option<ClayTileBlock>,

}

impl ZoneEntity {
    pub fn get_position(&self) -> Vec3 {
        self.transform.translation
    }

    pub fn get_rotation_euler(&self) -> Vec3 {
        self.transform.rotation
    }

    pub fn get_scale(&self) -> Vec3 {
        self.transform.scale
    }

    pub fn get_custom_props(&self) -> &Option<CustomPropsMap> {
        &self.custom_props
    }

    pub fn from_entity_ref(
        entity_ref: &EntityRef 
        ) -> Option<Self> {

        let Some(name_comp) = entity_ref.get::<Name>() else {return None};
        let Some(xform) = entity_ref.get::<Transform>() else {return None};
        let custom_props_component = entity_ref.get::<CustomPropsComponent>() ;
        let clay_tile_block_data = entity_ref.get::<ClayTileBlock>() ;

      //  if let Some((name, xform, custom_props_component, clay_tile_block_data)) = zone_entity_query.get(entity).ok() {
            let custom_props = custom_props_component.and_then(|comp| Some(comp.props.clone()));

            return Some(Self {
                name: name_comp.as_str().to_string(),
                transform: xform.clone().into(),
                custom_props,
                clay_tile_block_data: clay_tile_block_data.cloned()
            });
     //   }

      //  None

    }

    //clean this up and move to root .. 
    /*fn from_entity(
        entity: Entity,
        zone_entity_query: &Query<(&Name, &Transform, Option<&CustomPropsComponent>, Option<&ClayTileBlock>)>,
    ) -> Option<Self> {
        if let Some((name, xform, custom_props_component, clay_tile_block_data)) = zone_entity_query.get(entity).ok() {
            let custom_props = custom_props_component.and_then(|comp| Some(comp.props.clone()));

            return Some(Self {
                name: name.as_str().to_string(),
                transform: xform.clone().into(),
                custom_props,
                clay_tile_block_data: clay_tile_block_data.cloned()
            });
        }

        None
    }*/
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformSimple {
    pub translation: Vec3,
    pub rotation: Vec3, //euler
    pub scale: Vec3,
}

impl From<Transform> for TransformSimple {
    fn from(transform: Transform) -> Self {
        // Extract translation directly
        let translation = transform.translation;

        // Convert quaternion to Euler angles (in radians)
        let ( yaw, pitch,  roll) = transform.rotation.to_euler(EulerRot::YXZ);

        // Extract scale directly
        let scale = transform.scale;

        // Create and return a new instance of TransformSimple
        TransformSimple {
            translation,
            rotation: Vec3::new(yaw, pitch, roll ), 
            scale,
        }
    }
}
