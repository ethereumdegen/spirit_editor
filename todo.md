

## Spirit Editor





- move prefabs to game assets + load from artifacts 


- The way that material name importing works FOR MATERIAL SWAP  is borked.... the way it uses hashmap ? (use the new built in GltfMaterialName comp in next bevy version )
 -  I want to use the same glb file for multiple doodads 
 - in the next versiion of bevy, try to use the built in GltfMaterialName comp to fix this . 




- if a custom props component ever has a 'target_unique_name' then look for the thing w that unique name.. add a component for virtually linking 

 


    //BUG: loading model twice + override  breaks materials 

- bug: cant select units ? 
 BUG : painting terrain makes it invisible ? 

 BUG: Gizmo local offset is busted for making prefabs 
 
 
- synty swamp has an issue with texcoords ?


## Immediate todo 

 - improve look of house trim ! 
 
 
-  need to build prefabs at 0,0,0 until gizmo is fixed.. 
- if a prefab is Saved,  refresh other ones like it 

 - onc a prefab is placed, reset  selected prefab 
 
 1. add a transform offset to a zone 
   
 3. translating a prefab child should put the gizmo in the correct spot ... (bug in bevy_transform_gizmo)


- add visual grid snap dimensions (0.14 has grid gizmo)


- remember can  handle controls better  by  using bevy_editor_pls/src/controls.rs




- allow for disable shadows in editor ??

- add doodad bundles (house) 

-  add snap dimensions 

- when cloning an asset, auto select the new clone 

- clean up code / refactor (pls as base ? ) (less hacking of hierarchy ?)
 
 
## LATER  


- BIG  upgrade to UI -- put ui tools in Root and get rid of egui (PLS) 

-build a separate ActiveZone window (click to select active zone) - dont use hierarchy for this 




## bugs 

 