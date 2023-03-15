use std::ops::{Add, Deref};

use bevy::prelude::{App, AssetServer, Changed, Component, Plugin, Res, With};
use iyes_loopless::prelude::{ConditionSet, IntoConditionalSystem};

use k_utils::{util_stages::UPDATE_DELETED, util_state::UtilState};

use bevy::{
    prelude::{Added, Color, Commands, Entity, Query, Transform, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
};

use super::{
    logic::{Dimensions, Position, Square},
    plugin::Game,
};

const SQUARE_SIZE: f32 = 100.;
const SPACING: f32 = SQUARE_SIZE;

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
            texture: asset_server.load(if (position.x + position.y) % 2 == 0 {
                "images/plain-square-white.png"
            } else {
                "images/plain-square-brown.png"
            }),
            transform: Transform {
                translation: Vec3::new(
                    (position.x as f32 - (dimensions.width - 1) as f32 / 2.) * SPACING,
                    (position.y as f32 - (dimensions.height - 1) as f32 / 2.) * SPACING,
                    0.,
                ),
                ..Default::default()
            },
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
                .into(),
        );
    }
}
