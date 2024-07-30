##  Spirit Editor 

 
A level editor for 3D worlds for the Bevy game engine.  


  ![image](https://github.com/ethereumdegen/bevy_mesh_terrain_editor/assets/6249263/cfea97c5-b73a-4a54-9e27-e1f0a5c36229)


  ![image](https://github.com/ethereumdegen/bevy_mesh_terrain_editor/assets/6249263/9e32f8a0-e513-4ee0-8b4b-3e4d73ab8608)



## Getting Started 

1. In order to boot up the editor successfully the first time, Make sure you go into assets/editorconfig and set it to load the sample zone as follows:

```

( 

initial_zones_to_load: Some([
       
      "sample_zone.zone"
 ]), 
  

) 

```

This will use load the sample zone with a single cube.   If you try to load the other zones, it may not work as you wont have all of the 'Doodad Manifest' files configured for those zones.  Doodad Manifest files tell the editor how to render the models of doodads by a lookup table (Hashmap) of the doodad name.  





## Tool Modes 
1. Terrain Height
2. Terrain Splat
3. Foliage (warbler grass)  ( disabled for now )
4. Regions
5. Doodad Placement (Press ESC to toggle) 




### Terrain Splat Map
 
- When painting splat, the system supports up to 255 textures with high efficiency and very few draw calls. However, the tradeoff is you have to be careful how you blend and paint them.  To blend, be sure that you use the 'layer fade' and fade between two textures at every transition or you will get artifact lines.  This technique does make painting slightly more tedius but offers extreme splat map optimization and texture capacity in your game. 



 
## Doodads and zone files 
 
Place doodads in zone files (sub-scenes) which can be saved and loaded.  These are spawnable entities, used for anything like rocks, trees, even structural GLTFs for your game.  These each have a name, a translation, and CustomProps (a hashmap of values: [ vec3,f32,i32,String ....]).  In this way, you can load them into your game the way that you need specifically.    


![level_blockout_1](https://github.com/ethereumdegen/bevy_mesh_terrain_editor/assets/6249263/63988249-0758-4518-a51c-b0c6a25bf2b4)




### Placing Doodads 

 - Create a file at assets/doodad_manifests/doodad_manifest.manifest.ron  and add your doodad definitions to that folder.  By default, this folder and the models folder are gitignored due to asset licensing.

```
 # this is an example doodad manifest file telling the editor how to render (preview) doodads 
 
  (
    doodad_definitions: [
        (
            name: "birch_yellow_1",
            model: GltfModel("models/doodads/birch_yellow.glb"),
        ) ,
        (
            name: "bonfire",
            model: GltfModel("models/doodads/bonfire.glb"),            ,
            initial_custom_props: Some({ "my_prop": Float(1.0) })
        ) 
    ]
  )


```




### Zones 

- A zone is an array of doodads that can be saved or loaded.  An entire level could be stored in a zone or just a subsection of a level; however you prefer.  Terrain is actually not stored in a zone but separately in terrain heightmap and splat files (images).

- Using the Zones window, you can spawn a zone entity.  Right click on this zone entity in the hierarchy to set it as the primary zone.  When a zone is primary, placed doodads will become children of the zone


- Right click on the zone entity in the hierarchy to **save the zone to a file**.  You can use the zone window to load zone files back in later.   This is the ONLY way to save the doodads that you place.  


 

### Exporting files to your game 

1. You will need to copy the terrain data folder from the assets folder of this editor to the assets folder of your game after you save all chunks.  Then, in your game, use the bevy_mesh_terrain crate to load the terrain from those image and binary files for heightmap, splat and collision.

2. You will need to save your zone files (right click on a zone in the hierarchy and click SAVE ZONE) and then move those files over to your game.  I maintain custom import scripts in rust in my games codebase to parse the zone files so my game will know how to spawn the doodads when the player enters the game scene . However, the process is of course very similar to how this editor loads the zone files and spawns in the entities and models from them. 







