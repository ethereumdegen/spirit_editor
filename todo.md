

## Bedy Mod Edit 




## simple way to begin 

- use bevys hot reloading  for the terrain repainting.. when i update the image it should refresh the render material . 
- also.. need a splat map for each 'tile' or 'chunk' so that means each will need a different material ultimately... 


## Thoughts

Probably need a way to instruct the bevy_mesh_terrain plugin to save and load its data, indexed by 'tile' or 'chunk' .... save chunk X or load chunk X to/from file

Probably need a way to be able to edit/paint heightmap data which is in memory -- MAYBE the editor keeps track of (caches) this data in its memory  instead of b_m_t caching it 