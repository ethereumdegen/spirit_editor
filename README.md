##  Spirit Editor 

 
A level editor for 3D worlds for the Bevy game engine.  


  ![image](https://github.com/ethereumdegen/bevy_mesh_terrain_editor/assets/6249263/cfea97c5-b73a-4a54-9e27-e1f0a5c36229)


  ![image](https://github.com/ethereumdegen/bevy_mesh_terrain_editor/assets/6249263/9e32f8a0-e513-4ee0-8b4b-3e4d73ab8608)



## Getting Started 

1. In order to boot up the editor successfully the first time, Make sure you go into assets/editorconfig and set it to load the sample level as follows:

```

( 

initial_level_to_load: Some([
       
      "default_level.level"
 ]), 
  

) 

```

This will use load the sample zone with a single cube.   If you try to load the other zones, it may not work as you wont have all of the 'Doodad Manifest' files configured for those zones.  Doodad Manifest files tell the editor how to render the models of doodads by a lookup table (Hashmap) of the doodad name.  



2. Eventually, you need to edit the "external_game_assets_folder" parameter in editorconfig:

```
 external_game_assets_folder: Some("../healing-spirit-game-assets/game-assets"),

```

For now, set it to None. If you set it to None, then it will work and will only load a default sample doodad manifest.  In order to actually place more types of doodads in your editor, you will need to point this at a repo path which contains a subfolder "doodad_manifests" which contains all of your doodad manifest files.  (spirit_edit_core/src/doodads/doodad_manifest.rs has the struct to show you the architecture for this RON file ) (an example is found in example_game_assets).

On boot up of the editor, the contents of this external game assets repo will be copied into the 'artifacts' folder which is .gitignored.  In this way, the assets for the game you are building can remain separate from the assets of the editor itself. 

See 'example_game_assets' for an example of how this private repo should be set up.  Typically, it contains doodad_manifest files and all models for those doodads for your game.  For example, the doodad manifest would specify "tree_1" as a doodad type and then you would have a model "tree_1.glb" in the models folder.  When the editor boots, all of that gets copied to /artifacts/game_assets automatically and all of that data is used to build the "doodads pane".  


## Tool Modes 
1. Terrain Height
2. Terrain Splat
3. Foliage (warbler grass)  ( disabled for now )
4. Regions
5. Doodad Placement (Press ESC to toggle) 




### Terrain Splat Map
 
- You can configure the look and feel of your terrain that you paint by editing the contents of assets/terrain/TERRAIN_NAME .  Once you are happy with your terrain configuration and painted/heightmapped data, simply copy the terrain files to your game assets and load it the same way in your game.  



 
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



### Warning : Assets

Any texture or GLTF assets (in the assets folder) that are in this repo (that may have been checked in on purpose or accidentally) are NOT to be used by you in any way due to licensing.  You are responsible for adding your own texture and GLTF assets to the manifests. 



