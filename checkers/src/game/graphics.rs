use bevy::prelude::{App, AssetServer, Handle, Image, Plugin, Res};
use iyes_loopless::prelude::ConditionSet;

use k_utils::{util_stages::UPDATE_DELETED, util_state::UtilState};

use bevy::{
    prelude::{Added, Commands, Entity, Query, Transform, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
};

use super::{
    logic::{Dimensions, Owner, Piece, Position, Square},
    plugin::Game,
};

const SQUARE_SIZE: f32 = 100.;
const SPACING: f32 = SQUARE_SIZE;
const PIECE_SIZE: f32 = 75.;

fn pos_to_transform(pos: &Position, dim: &Dimensions, z: f32) -> Transform {
    Transform {
        translation: Vec3::new(
            (pos.x as f32 - (dim.width - 1) as f32 / 2.) * SPACING,
            (pos.y as f32 - (dim.height - 1) as f32 / 2.) * SPACING,
            z,
        ),
        ..Default::default()
    }
}

fn owner_to_image(piece: &Piece, asset_server: &AssetServer) -> Handle<Image> {
    asset_server.load(match piece.owner {
        Owner::White => "images/white-disk.png",
        Owner::Black => "images/black-disk.png",
    })
}

fn square_to_image(position: &Position, asset_server: &AssetServer) -> Handle<Image> {
    asset_server.load(if (position.x + position.y) % 2 == 0 {
        "images/plain-square-white.png"
    } else {
        "images/plain-square-brown.png"
    })
}

fn on_add(
    mut commands: Commands,
    new_squares: Query<(Entity, &Position), Added<Square>>,
    dimensions: Res<Dimensions>,
    asset_server: Res<AssetServer>,
) {
    for (entity, position) in new_squares.iter() {
        commands.entity(entity).insert(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ..Default::default()
            },
            texture: square_to_image(position, &asset_server),
            transform: pos_to_transform(position, &dimensions, 0.),
            ..Default::default()
        });
    }
}

fn on_add_piece(
    mut commands: Commands,
    new_squares: Query<(Entity, &Position, &Piece), Added<Piece>>,
    dimensions: Res<Dimensions>,
    asset_server: Res<AssetServer>,
) {
    for (entity, position, piece) in new_squares.iter() {
        commands.entity(entity).insert(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(PIECE_SIZE, PIECE_SIZE)),
                ..Default::default()
            },
            texture: owner_to_image(piece, &asset_server),
            transform: pos_to_transform(position, &dimensions, 1.),
            ..Default::default()
        });
    }
}

#[derive(Default)]
pub struct Graphics {}

impl Plugin for Graphics {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            UPDATE_DELETED,
            ConditionSet::new()
                .run_in_state(UtilState::<Game>::Running)
                .with_system(on_add)
                .with_system(on_add_piece)
                .into(),
        );
    }
}
