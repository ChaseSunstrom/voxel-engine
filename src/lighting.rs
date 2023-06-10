use bevy::math::Vec3;
use bevy::prelude::{Color, Commands, DirectionalLight, DirectionalLightBundle, Transform};
use bevy::utils::default;

pub fn create_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}
