use std::time::Duration;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, time::common_conditions::on_timer};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::pbr::DirectionalLightShadowMap;
use bevy_flycam::NoCameraPlayerPlugin;

use crate::{camera, fps, lighting, render};

//these are useless at the moment
const _WINDOW_HEIGHT: f32 = 1280.0;
const _WINDOW_WIDTH: f32 = 1920.0;

pub fn start() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.53, 0.8, 0.92)))
        .insert_resource(DirectionalLightShadowMap { size: 2048})
        .add_plugins(DefaultPlugins)
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(camera::init_camera)
        .add_startup_system(render::render_cubes)
        .add_startup_system(fps::fps_setup)
        .add_startup_system(lighting::create_light)
        .add_system(fps::fps_system.run_if(on_timer(Duration::from_secs_f32(0.5))))
        .run();
}
