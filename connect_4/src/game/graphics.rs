use std::ops::Deref;

use alpha_beta::GameModel;
use bevy::prelude::{App, Changed, Component, Plugin, Res, With};
use connect_4_model::{
    types::{Owner, Player},
    Model, Move,
};
use iyes_loopless::prelude::{ConditionSet, IntoConditionalSystem};

use super::{
    input::{Column, Human},
    logic::{MoveHistory, OwnerW, PositionW},
};
use k_utils::{
    raycast::components::GameInteraction,
    util_graphics::update_graphics,
    util_stages::UPDATE_DELETED,
    util_state::{StateContraint, UtilState},
};

use bevy::{
    prelude::{Added, Color, Commands, Entity, Query, Transform, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
};

const SQUARE_SIZE: f32 = 80.;
const SPACING: f32 = SQUARE_SIZE + 10.;

fn on_add(
    mut commands: Commands,
    move_history: Res<MoveHistory>,
    new_squares: Query<(Entity, &PositionW), Added<PositionW>>,
) {
    let board = Model::from(move_history.0.iter());
    for (entity, position) in new_squares.iter() {
        commands.entity(entity).insert(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(
                    (position.x as f32 - (board.dimensions.0 - 1) as f32 / 2.) * SPACING,
                    (position.y as f32 - (board.dimensions.1 - 1) as f32 / 2.) * SPACING,
                    0.,
                ),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

fn on_add_column(
    mut commands: Commands,
    move_history: Res<MoveHistory>,
    new_columns: Query<(Entity, &Column), Added<Column>>,
) {
    let board = Model::from(move_history.0.iter());
    for (entity, column) in new_columns.iter() {
        commands
            .entity(entity)
            .insert(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0., 0., 0., 0.),
                    custom_size: Some(Vec2::new(SPACING, board.dimensions.1 as f32 * SPACING)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        (column.0 as f32 - (board.dimensions.0 - 1) as f32 / 2.) * SPACING,
                        0.,
                        0.,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(GameInteraction::new());
    }
}

#[derive(Component)]
struct Highlighted;

use k_utils::raycast::components::Interaction::*;

fn update_proposed_color(
    mut commands: Commands,
    mut board_query: Query<(Entity, &mut Sprite, &PositionW, &OwnerW)>,
    current_highlighted: Query<Entity, With<Highlighted>>,
    interactions: Query<(&Column, &GameInteraction), Changed<GameInteraction>>,
    move_history: Res<MoveHistory>,
    humans: Query<&Human>,
) {
    if interactions.is_empty() {
        return;
    }
    if let Some(entity) = current_highlighted.iter().next() {
        if let Ok(mut square) = board_query.get_mut(entity) {
            square.1.color = square.3.to_color();
        }
        commands.entity(entity).remove::<Highlighted>();
    }
    let board = Model::from(move_history.0.iter());
    if humans.iter().all(|ap| board.active_player.ne(&ap.0)) {
        return;
    }
    let moves = board.legal_moves();
    for (&pos, &GameInteraction { interaction }) in interactions.iter() {
        if let Some(m) = moves
            .iter()
            .filter(|Move(p, _)| p.x == pos.0 as usize)
            .next()
        {
            match interaction {
                Hover | JustClicked | Clicked => {
                    for (entity, mut sprite, position, _) in board_query.iter_mut() {
                        if position.deref().eq(&m.0) {
                            sprite.color = m.1.to_color();
                            commands.entity(entity).insert(Highlighted);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn update_color(mut board: Query<(&mut Sprite, &OwnerW), Changed<OwnerW>>) {
    for (mut sprite, &owner) in board.iter_mut() {
        sprite.color = match *owner {
            Owner::None => Color::WHITE,
            Owner::Owned(player) => player.to_color(),
        }
    }
}

trait ToColor {
    fn to_color(self) -> Color;
}

impl ToColor for Owner {
    fn to_color(self) -> Color {
        match self {
            Owner::Owned(player) => player.to_color(),
            Owner::None => Color::WHITE,
        }
    }
}

impl ToColor for Player {
    fn to_color(self) -> Color {
        match self {
            Player::Red => Color::RED,
            Player::Blue => Color::BLUE,
        }
    }
}

#[derive(Default)]
pub struct Graphics<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: StateContraint + Sync + Send> Plugin for Graphics<T> {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            UPDATE_DELETED,
            ConditionSet::new()
                .run_in_state(UtilState::<T>::Running)
                .with_system(on_add)
                .with_system(on_add_column)
                .into(),
        );
        update_graphics::<T>(
            app,
            vec![
                update_color.into_conditional(),
                update_proposed_color.into_conditional(),
            ],
        );
    }
}
