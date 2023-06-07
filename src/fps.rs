use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

const FPS_SECTION_INDEX: usize = 0;

#[derive(Component)]
pub struct FpsText;

pub fn fps_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "FPS: 120",
            TextStyle {
                // https://www.dafont.com/vcr-osd-mono.font
                font: asset_server.load("fonts/VCR_OSD_MONO_1.001.ttf"),
                font_size: 42.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        FpsText,
    ));
}

pub fn fps_system(
    diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>
) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps) = fps.average() {
            let mut fps_text = query.single_mut();
            fps_text.sections[FPS_SECTION_INDEX].value = format!("FPS: {}", fps.round());
        }
    }
}
