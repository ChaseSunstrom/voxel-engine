use bevy::prelude::*;
use bevy_flycam::{NoCameraPlayerPlugin, FlyCam};

//not sure why intelliJ doesn't recognize the bevy_flycam plugin
pub fn init_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle::default(), FlyCam));
}