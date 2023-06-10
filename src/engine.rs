use std::time::Duration;

use bevy::pbr::DirectionalLightShadowMap;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, time::common_conditions::on_timer};
use bevy_atmosphere::prelude::*;
use bevy_flycam::NoCameraPlayerPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{camera, fps, lighting, render, voxel};

pub fn start() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.53, 0.8, 0.92)))
        //.insert_resource(DirectionalLightShadowMap { size: 2048 })
        .add_plugins(DefaultPlugins)
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(AtmospherePlugin)
        //.add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(camera::init_camera)
        //.add_startup_system(render::render_cubes)
        .add_startup_system(fps::fps_setup)
        .add_startup_system(voxel::add_voxel_material)
        .add_startup_system(voxel::spawn_voxel)
        //.add_startup_system(lighting::create_light)
        .add_system(fps::fps_system.run_if(on_timer(Duration::from_secs_f32(0.5))))
        .run();
}
