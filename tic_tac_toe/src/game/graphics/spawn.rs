use bevy::{
    prelude::{
        shape, AssetServer, Assets, BuildChildren, Changed, Children, Commands, Entity,
        EventReader, EventWriter, Handle, Mesh, Quat, Query, Res, ResMut, Transform, Vec2, Vec3,
        With,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    time::Time,
};

use crate::{
    game::model::{
        events::ResetBoard,
        types::{winner, Player, Square},
    },
    raycast::components::GameInteraction,
};

use super::{
    brush::{Animated, BrushMaterial},
    events::{Animate, Despawn, Spawn},
    square::to_vec,
};

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<BrushMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut spawn_er: EventReader<Spawn>,
) {
    for spawn in spawn_er.iter() {
        println!("Spawning!");
        match spawn.owner {
            Player::X => {
                spawn_x(
                    &mut commands,
                    &asset_server,
                    &mut materials,
                    &mut meshes,
                    spawn.parent,
                );
            }
            Player::O => {
                spawn_o(
                    &mut commands,
                    &asset_server,
                    &mut materials,
                    &mut meshes,
                    spawn.parent,
                );
            }
        };
    }
}

pub fn despawn(
    mut commands: Commands,
    mut despawn_ew: EventReader<Despawn>,
    brushes: Query<Entity, With<Mesh2dHandle>>,
    children: Query<&Children>,
) {
    for despawn in despawn_ew.iter() {
        println!("Despawning!");
        if let Ok(children) = children.get(despawn.parent) {
            for &child in children {
                if brushes.contains(child) {
                    commands.entity(child).despawn();
                }
            }
        }
    }
}

pub fn clear_lines(
    mut commands: Commands,
    reset_er: EventReader<ResetBoard>,
    mut meshes: Query<Entity, With<Mesh2dHandle>>,
) {
    if reset_er.is_empty() {
        return;
    }
    for entity in meshes.iter_mut() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_o(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<BrushMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    parent: Entity,
) {
    let mesh_handle: Mesh2dHandle = meshes
        .add(Mesh::from(shape::Quad::new(Vec2::new(200., 200.))))
        .into();

    commands.entity(parent).add_children(|parent| {
        parent.spawn(MaterialMesh2dBundle {
            mesh: mesh_handle.clone(),
            material: materials.add(BrushMaterial::o_brush(asset_server)),
            ..Default::default()
        });
    });
}

pub fn spawn_x(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<BrushMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    parent: Entity,
) {
    commands.entity(parent).add_children(|parent| {
        let mesh_handle: Mesh2dHandle = meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(200., 33.))))
            .into();
        parent.spawn(MaterialMesh2dBundle {
            mesh: mesh_handle.clone(),
            material: materials.add(BrushMaterial::line_brush(&asset_server, 0.3)),
            transform: Transform::from_rotation(Quat::from_rotation_z(
                -std::f32::consts::FRAC_PI_4,
            )),
            ..Default::default()
        });
        let mesh_handle: Mesh2dHandle = meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(200., 33.))))
            .into();
        parent.spawn(MaterialMesh2dBundle {
            mesh: mesh_handle.clone(),
            material: materials.add(BrushMaterial::line_brush(&asset_server, 0.0)),
            transform: Transform::from_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4)),
            ..Default::default()
        });
    });
}

pub fn animate(
    mut commands: Commands,
    parents: Query<&Children>,
    brushes: Query<&Handle<BrushMaterial>>,
    mut animate_er: EventReader<Animate>,
) {
    for animate in animate_er.iter() {
        if let Ok(children) = parents.get(animate.0) {
            for &child in children {
                if brushes.contains(child) {
                    commands.entity(child).insert(Animated);
                }
            }
        }
    }
}

pub fn update_time(
    mut brushes: Query<&Handle<BrushMaterial>, With<Animated>>,
    mut materials: ResMut<Assets<BrushMaterial>>,
    time: Res<Time>,
) {
    for brush in brushes.iter_mut() {
        if let Some(material) = materials.get_mut(brush) {
            material.increment_time(&time);
        }
    }
}

pub fn check_win(
    mut commands: Commands,
    changed: Query<Entity, Changed<Square>>,
    squares: Query<&Square>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<BrushMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if changed.is_empty() {
        return;
    }
    let all_squares = squares.iter().copied().collect::<Vec<_>>().into_iter();
    let winner = winner(all_squares);
    if let Some(lines) = winner.1 {
        for (i, line) in lines.iter().enumerate() {
            let (start, end) = (line.0.first().unwrap(), line.0.last().unwrap());
            let (start, end) = (to_vec(*start), to_vec(*end));
            let mid = (start + end) / 2. + Vec3::Z;
            let diff = end - start;
            let length = diff.length();
            let rotation = Quat::from_rotation_z(f32::atan2(diff.y, diff.x));

            let mesh_handle: Mesh2dHandle = meshes
                .add(Mesh::from(shape::Quad::new(Vec2::new(1.6 * length, 20.))))
                .into();

            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: mesh_handle.clone(),
                    material: materials.add(BrushMaterial::long_line_brush(
                        &asset_server,
                        -0.3 - 0.3 * (i as f32),
                    )),
                    transform: Transform {
                        translation: mid,
                        rotation,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Animated);
        }
    }
}

pub fn square_updated(
    mut commands: Commands,
    changed: Query<(Entity, &Square), Changed<Square>>,
    mut spawn_ew: EventWriter<Animate>,
) {
    for (entity, &Square { owner, .. }) in changed.iter() {
        if owner.is_some() {
            commands.entity(entity).remove::<GameInteraction>();
            spawn_ew.send(Animate(entity));
        }
    }
}
