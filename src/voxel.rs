use super::constants::Constants;
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    transform,
    utils::HashMap,
};
use enum_iterator::Sequence;
use rand::Rng;

#[derive(Sequence, Clone, Copy, PartialEq, Eq, Hash)]
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
    Tnt,
    Oak,
}

impl Kind {
    // TODO: put those in a file?
    fn get_texture_coordinates(&self) -> HashMap<Face, (u8, u8)> {
        match self {
            Kind::Tnt => enum_iterator::all()
                .map(|face| {
                    (
                        face,
                        match face {
                            Face::Front => (8, 0),
                            Face::Back => (8, 0),
                            Face::Right => (8, 0),
                            Face::Left => (8, 0),
                            Face::Top => (9, 0),
                            Face::Bottom => (10, 0),
                        },
                    )
                })
                .collect(),
            Kind::Oak => enum_iterator::all()
                .map(|face| {
                    (
                        face,
                        match face {
                            Face::Front => (4, 1),
                            Face::Back => (4, 1),
                            Face::Right => (4, 1),
                            Face::Left => (4, 1),
                            Face::Top => (5, 1),
                            Face::Bottom => (5, 1),
                        },
                    )
                })
                .collect(),
        }
    }
}

#[derive(Component)]
struct Voxel {
    kind: Kind,
}

#[derive(Bundle)]
struct VoxelBundle {
    voxel: Voxel,
    #[bundle]
    spacial: SpatialBundle,
}

#[derive(Resource)]
pub struct VoxelMetadata {
    uv_offset: Vec2,
    voxel_offset: f32,
    texture: Handle<Image>,
    default_material: Handle<StandardMaterial>,
}

pub fn load_voxel_metadata(
    constants: Res<Constants>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture = asset_server.load(&constants.voxel_texture_path);
    let default_material = materials.add(StandardMaterial {
        base_color_texture: Some(texture.clone()),
        ..default()
    });
    let voxel_texture_x_amount = 16;
    let voxel_texture_y_amount = 16;
    let uv_offset = 1.0 / Vec2::new(voxel_texture_x_amount as f32, voxel_texture_y_amount as f32);

    commands.insert_resource(VoxelMetadata {
        uv_offset,
        voxel_offset: constants.voxel_size / 2.0,
        texture,
        default_material,
    });
}

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    constants: Res<Constants>,
    voxel_metadata: Res<VoxelMetadata>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keys.just_pressed(KeyCode::Return) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-5..5) as f32;
        let y = rng.gen_range(-5..5) as f32;
        let z = rng.gen_range(-30..-20) as f32;
        let transform = Transform::from_xyz(x, y, z);
        spawn_voxel(transform, &voxel_metadata, &mut commands, &mut meshes);
    }
}

fn spawn_voxel(
    transform: Transform,
    voxel_metadata: &VoxelMetadata,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let kind = if rand::random() { Kind::Tnt } else { Kind::Oak };
    let texture_coordinates = kind.get_texture_coordinates();
    commands
        .spawn(VoxelBundle {
            voxel: Voxel { kind },
            spacial: SpatialBundle {
                transform,
                ..default()
            },
        })
        .with_children(|parent| {
            for face in enum_iterator::all() {
                parent.spawn(create_voxel_face(
                    face,
                    *texture_coordinates.get(&face).unwrap(),
                    voxel_metadata,
                    meshes,
                ));
            }
        });
}

type Positions = Vec<[f32; 3]>;
type Normals = Vec<Vec3>;
type Uvs = Vec<Vec2>;
type Vertices = [([f32; 3], Vec3, Vec2)];

fn create_voxel_face(
    face: Face,
    texture_coordinates: (u8, u8),
    voxel_metadata: &VoxelMetadata,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> VoxelFaceBundle {
    let offset = voxel_metadata.voxel_offset;
    let uv_offset = voxel_metadata.uv_offset;
    let voxel_material = voxel_metadata.default_material.clone();
    let texture_coordinates = Vec2::new(texture_coordinates.0 as f32, texture_coordinates.1 as f32);

    let top_left = (Vec2::ZERO + texture_coordinates) * uv_offset;
    let top_right = (Vec2::X + texture_coordinates) * uv_offset;
    let bottom_left = (Vec2::Y + texture_coordinates) * uv_offset;
    let bottom_right = (Vec2::ONE + texture_coordinates) * uv_offset;

    let vertices = &match face {
        Face::Front => [
            ([-offset, -offset, offset], Vec3::Z, bottom_left),
            ([offset, -offset, offset], Vec3::Z, bottom_right),
            ([-offset, offset, offset], Vec3::Z, top_left),
            ([offset, offset, offset], Vec3::Z, top_right),
        ],
        Face::Back => [
            ([-offset, offset, -offset], Vec3::NEG_Z, top_right),
            ([offset, offset, -offset], Vec3::NEG_Z, top_left),
            ([-offset, -offset, -offset], Vec3::NEG_Z, bottom_right),
            ([offset, -offset, -offset], Vec3::NEG_Z, bottom_left),
        ],
        Face::Right => [
            ([offset, -offset, -offset], Vec3::X, bottom_right),
            ([offset, offset, -offset], Vec3::X, top_right),
            ([offset, -offset, offset], Vec3::X, bottom_left),
            ([offset, offset, offset], Vec3::X, top_left),
        ],
        Face::Left => [
            ([-offset, -offset, offset], Vec3::NEG_X, bottom_right),
            ([-offset, offset, offset], Vec3::NEG_X, top_right),
            ([-offset, -offset, -offset], Vec3::NEG_X, bottom_left),
            ([-offset, offset, -offset], Vec3::NEG_X, top_left),
        ],
        Face::Top => [
            ([offset, offset, -offset], Vec3::Y, top_right),
            ([-offset, offset, -offset], Vec3::Y, top_left),
            ([offset, offset, offset], Vec3::Y, bottom_right),
            ([-offset, offset, offset], Vec3::Y, bottom_left),
        ],
        Face::Bottom => [
            ([offset, -offset, offset], Vec3::NEG_Y, top_right),
            ([-offset, -offset, offset], Vec3::NEG_Y, top_left),
            ([offset, -offset, -offset], Vec3::NEG_Y, bottom_right),
            ([-offset, -offset, -offset], Vec3::NEG_Y, bottom_left),
        ],
    };
    let (positions, normals, uvs) = destructure_vertices(vertices);
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleStrip);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(vec![0, 1, 2, 3])));

    VoxelFaceBundle {
        voxel_face: VoxelFace { face },
        pbr: PbrBundle {
            material: voxel_material,
            mesh: meshes.add(mesh),
            ..default()
        },
    }
}

fn destructure_vertices(vertices: &Vertices) -> (Positions, Normals, Uvs) {
    let capacity = vertices.len();
    let (mut positions, mut normals, mut uvs) = (
        Vec::with_capacity(capacity),
        Vec::with_capacity(capacity),
        Vec::with_capacity(capacity),
    );
    for (position, normal, uv) in vertices {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);
    }
    (positions, normals, uvs)
}
