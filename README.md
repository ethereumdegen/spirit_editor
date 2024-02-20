## Bevy Mesh Terrain Editor 

Load, Edit, and Save terrain files for bevy_mesh_terrain in a standalone application 



### How to use 



### Tips and tricks 

- You dont have to 'save all chunks' unless you need to export collision data to a game.  Often, saving splat and height is sufficient and far faster. 

- When painting, the system supports up to 255 textures. However, you have to  be very careful how you blend them.  To blend, be sure that you use the 'layer fade' and fade between two textures at every transition or you will get artifact lines.  This technique does make painting slightly more tedius but offers extreme splat map optimization and texture capacity in your game. 


