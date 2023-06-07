use bevy::prelude::*;

//no clue what this does
pub fn render_cubes(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>) {
    let mesh = mesh_assets.add(shape::Box::new(1.0, 1.0, 1.0).into());
    for x in -10..10 {
        for z in -10..10 {
            for y in -10..10 {
                commands.spawn(PbrBundle {
                    mesh: mesh.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * 2.,
                        y as f32 * 2.,
                        z as f32 * 2.,
                    )),
                    ..Default::default()
                });
            }
        }
    }
}
