use std::ops;
use bevy::prelude::*;
use rand::Rng;

//no clue what this does
pub fn render_cubes(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();

    let mesh = mesh_assets.add(shape::Box::new(1.0, 1.0, 1.0).into());
    let material = material_assets.add(StandardMaterial {
        base_color: Color::rgb(0.2, 0.8, 0.2),
        ..default()
    });

    for x in -10..10 {
        for z in -10..10 { 
            for y in -10..10 {
                commands.spawn(PbrBundle {
                    mesh: mesh.clone(),
                    transform: Transform::from_xyz(x as f32*rng.gen::<f32>(), y as f32*rng.gen::<f32>(), z as f32*rng.gen::<f32>()),
                    material: material.clone(),
                    ..default()
                });
            }
        }
    }
}