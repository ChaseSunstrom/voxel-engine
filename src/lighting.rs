use bevy::math::Vec3;
use bevy::prelude::{
    Color, Commands, DirectionalLight, DirectionalLightBundle, EulerRot, Quat, Transform,
};
use bevy::utils::default;

pub fn create_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            (20.0_f32).to_radians(),
            (10.0_f32).to_radians(),
            (30.0_f32).to_radians(),
        )),
        ..default()
    });
}
