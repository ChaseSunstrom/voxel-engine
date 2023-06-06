use bevy::app::App;
use bevy::DefaultPlugins;


pub fn start() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}