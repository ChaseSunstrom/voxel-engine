use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    transform,
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
    // TODO: provide this as a parameter
    let transform = Transform::from_xyz(0.0, 0.0, -8.0);
    commands
        .spawn(VoxelBundle {
            voxel: Voxel { kind: Kind::Grass },
            spacial: SpatialBundle {
                transform,
                ..default()
            },
        })
        .with_children(|parent| {
            parent.spawn(create_voxel_face(
                Face::Bottom,
                meshes,
                materials,
                //voxel_material,
            ));
        });
}

// TODO: put into some "global resource"
const VOXEL_SIZE: f32 = 1.0;

fn create_voxel_face(
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
    let offset = VOXEL_SIZE / 2.0;
    let vertices = &match face {
        Face::Front => [
            ([-offset, -offset, offset], Vec3::Z, Vec2::ZERO),
            ([offset, -offset, offset], Vec3::Z, Vec2::X),
            ([-offset, offset, offset], Vec3::Z, Vec2::Y),
            ([offset, offset, offset], Vec3::Z, Vec2::ONE),
        ],
        Face::Back => [
            ([-offset, offset, -offset], Vec3::NEG_Z, Vec2::X),
            ([offset, offset, -offset], Vec3::NEG_Z, Vec2::ZERO),
            ([-offset, -offset, -offset], Vec3::NEG_Z, Vec2::ONE),
            ([offset, -offset, -offset], Vec3::NEG_Z, Vec2::Y),
        ],
        Face::Right => [
            ([offset, -offset, -offset], Vec3::X, Vec2::ZERO),
            ([offset, offset, -offset], Vec3::X, Vec2::X),
            ([offset, -offset, offset], Vec3::X, Vec2::Y),
            ([offset, offset, offset], Vec3::X, Vec2::ONE),
        ],
        Face::Left => [
            ([-offset, -offset, offset], Vec3::NEG_X, Vec2::X),
            ([-offset, offset, offset], Vec3::NEG_X, Vec2::ZERO),
            ([-offset, -offset, -offset], Vec3::NEG_X, Vec2::ONE),
            ([-offset, offset, -offset], Vec3::NEG_X, Vec2::Y),
        ],
        Face::Top => [
            ([offset, offset, -offset], Vec3::Y, Vec2::ZERO),
            ([-offset, offset, -offset], Vec3::Y, Vec2::X),
            ([offset, offset, offset], Vec3::Y, Vec2::Y),
            ([-offset, offset, offset], Vec3::Y, Vec2::ONE),
        ],
        Face::Bottom => [
            ([offset, -offset, offset], Vec3::NEG_Y, Vec2::X),
            ([-offset, -offset, offset], Vec3::NEG_Y, Vec2::ZERO),
            ([offset, -offset, -offset], Vec3::NEG_Y, Vec2::ONE),
            ([-offset, -offset, -offset], Vec3::NEG_Y, Vec2::Y),
        ],
    };
    let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
    let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
    let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
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
