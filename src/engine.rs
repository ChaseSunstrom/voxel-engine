use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::time::TimePlugin;
use bevy_flycam::{NoCameraPlayerPlugin, FlyCam};

use crate::{camera, render};

//these are useless at the moment
const WINDOW_HEIGHT: f32 = 1280.0;
const WINDOW_WIDTH: f32 = 1920.0;

pub fn start() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.53, 0.8, 0.92)))
        .add_plugins(DefaultPlugins)
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(camera::init_camera)
        .add_startup_system(render::render_cubes)
        .run();
}