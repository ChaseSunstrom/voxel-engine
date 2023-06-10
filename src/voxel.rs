use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

enum Face {
    Front,
    Back,
    Right,
    Left,
    Top,
    Bottom,
}

#[derive(Component)]
struct VoxelFace {
    face: Face,
}

#[derive(Bundle)]
struct VoxelFaceBundle {
    voxel_face: VoxelFace,
    #[bundle]
    pbr: PbrBundle,
}

enum Kind {
    Grass,
}

#[derive(Component)]
struct Voxel {
    kind: Kind,
}

#[derive(Bundle)]
struct VoxelBundle {
    voxel: Voxel,
    #[bundle]
    // TODO: do we need Visibility for the voxel itself?
    spacial: SpatialBundle,
}

#[derive(Resource)]
pub struct VoxelMaterial(Handle<StandardMaterial>);

// TODO: use this to insert the resource
pub fn add_voxel_material(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    let voxel_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.2, 0.8, 0.2),
        ..default()
    });
    commands.insert_resource(VoxelMaterial(voxel_material));
}

pub fn spawn_voxel(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    //voxel_material: Res<VoxelMaterial>,
) {
    // TODO: provide this as a parameter and center voxel
    let position = Vec3::new(-0.5, -0.5, -8.0);
    commands
        .spawn(VoxelBundle {
            voxel: Voxel { kind: Kind::Grass },
            spacial: SpatialBundle {
                transform: Transform::from_translation(position),
                ..default()
            },
        })
        .with_children(|parent| {
            parent.spawn(create_voxel_face(
                position,
                Face::Front,
                meshes,
                materials,
                //voxel_material,
            ));
        });
}

fn create_voxel_face(
    voxel_position: Vec3,
    face: Face,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //voxel_material: Res<VoxelMaterial>,
) -> VoxelFaceBundle {
    // TODO: remove once create_voxel_face is no longer a startup system
    let voxel_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.2, 0.8, 0.2),
        ..default()
    });
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleStrip);
    // TODO: steal code from shape::Box to create faces
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            // TODO: add voxel size and use that for the correct offsets
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
        ],
    );
    mesh.set_indices(Some(Indices::U32(vec![0, 1, 2, 3])));
    VoxelFaceBundle {
        voxel_face: VoxelFace { face },
        pbr: PbrBundle {
            material: voxel_material,
            //material: voxel_material.0.clone(),
            mesh: meshes.add(mesh),
            ..default()
        },
    }
}
