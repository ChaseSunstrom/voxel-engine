use bevy::prelude::*;

#[derive(Resource)]
pub struct Constants {
    pub voxel_size: f32,
    pub voxel_texture_path: String,
    pub voxel_texture_mapping_path: String,
}

impl FromWorld for Constants {
    fn from_world(world: &mut World) -> Self {
        Constants {
            voxel_size: 1.0,
            // https://minecraft.fandom.com/wiki/Terrain.png
            voxel_texture_path: "textures/terrain.png".to_owned(),
            voxel_texture_mapping_path: "assets/textures/terrain_mapping.txt".to_owned(),
        }
    }
}
