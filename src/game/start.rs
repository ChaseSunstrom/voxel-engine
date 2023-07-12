use std::time::Duration;

use bevy::pbr::DirectionalLightShadowMap;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, time::common_conditions::on_timer};
//use bevy_atmosphere::prelude::*;
//use bevy_flycam::NoCameraPlayerPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{
    render::{
        camera::camera::*,
        world::{
            generation::voxel::*,
            light::lighting::*,
        },
    },
    util::{
        constants::*,
    },
    UI::fps::*,
};


pub fn start() {
    App::new()
        .init_resource::<Constants>()
        .insert_resource(ClearColor(Color::rgb(0.53, 0.8, 0.92)))
        .insert_resource(DirectionalLightShadowMap { size: 2048 })
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        //.add_plugin(NoCameraPlayerPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        //.add_plugin(AtmospherePlugin)
        //.add_startup_system(init_camera)
        //.add_startup_system(fps_setup)
        //.add_startup_system(load_voxel_metadata)
        //.add_system(keyboard_input)
        //.add_startup_system(create_light)
        //.add_system(fps_system.run_if(on_timer(Duration::from_secs_f32(0.5))))
        //.add_systems(Startup, (create_light, /*fps_setup,*/ init_camera, load_voxel_metadata))
        //.add_systems(Update, keyboard_input)
        .run();
}