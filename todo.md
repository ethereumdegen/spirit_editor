

## Bevy Mesh Terrain Editor




- bug: cant select units ? 
 BUG : painting terrain makes it invisible ? 
 
 
- synty swamp has an issue with texcoords ?


## Immediate todo 

 - make a dirt material  -- use for dirt cliffs !! 

 1. add more   texture to clay tiles  (addd normal maps )

 2. finish up the  cool rocks  ( make sure all rocks have uv ! and get their materials swapped )

 3. add terrain manifest to bevy_mesh_terrain and make it so each tile type has individually different UV expansion factor 
 4. make the terrain not disappear while painting .. 
 
5.  fix anceint forest   bushes  (like the trees) 

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

 