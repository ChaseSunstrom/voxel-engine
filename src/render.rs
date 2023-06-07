use bevy::pbr::MaterialProperties;
use bevy::prelude::*;
use bevy::render::render_resource::ShaderRef::Handle;


//no clue what this does
pub fn render_cubes(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>
) {
    let mut mesh = mesh_assets.add(shape::Box::new(1.0, 1.0, 1.0).into());
    let mut material = material_assets.add(StandardMaterial {
        base_color: Color::rgb(0.2, 0.8, 0.2),
        ..Default::default()
    });
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
                    material: material.clone(),
                    ..Default::default()
                });
            }
        }
    }
}
