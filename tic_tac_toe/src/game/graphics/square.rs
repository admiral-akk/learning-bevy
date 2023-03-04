use bevy::{
    prelude::{
        Added, Changed, Color, Commands, Entity, EventReader, EventWriter, Query, Transform, Vec2,
        Vec3, With,
    },
    sprite::{Sprite, SpriteBundle},
};

use crate::{
    game::{
        graphics::events::{Despawn, Spawn},
        model::{
            events::{Move, ResetBoard},
            types::{active_player, test_move, Position, Square},
        },
    },
    raycast::components::{GameInteraction, Interaction},
};

const SQUARE_SPACING: f32 = 140.;
pub const SQUARE_SIZE: f32 = 200.;

pub fn to_vec(position: Position) -> Vec3 {
    Vec3::new(
        position.0 as f32 * SQUARE_SIZE - SQUARE_SIZE,
        position.1 as f32 * SQUARE_SIZE - SQUARE_SIZE,
        0.,
    )
}

pub fn added_square(mut commands: Commands, added: Query<(Entity, &Square), Added<Square>>) {
    for (entity, square) in added.iter() {
        commands
            .entity(entity)
            .insert(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0., 0., 0., 0.),
                    custom_size: Some(Vec2::new(SQUARE_SPACING, SQUARE_SPACING)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: to_vec(square.position),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(GameInteraction::default());
    }
}

pub fn square_reset(
    mut commands: Commands,
    reset_er: EventReader<ResetBoard>,
    mut squares: Query<(Entity, &mut Square)>,
) {
    if reset_er.is_empty() {
        return;
    }
    println!("Resetting!");
    for (entity, mut square) in squares.iter_mut() {
        square.owner = None;
        commands.entity(entity).remove::<GameInteraction>();
        commands.entity(entity).insert(GameInteraction::new());
    }
}

pub fn square_update(
    mut squares: Query<&mut Square>,
    changed: Query<(Entity, &GameInteraction), (With<Square>, Changed<GameInteraction>)>,
    mut spawn_ew: EventWriter<Spawn>,
    mut move_ew: EventWriter<Move>,
    mut despawn_ew: EventWriter<Despawn>,
) {
    let all_squares = squares.iter().copied().collect::<Vec<_>>().into_iter();
    let active_player = active_player(all_squares.clone());
    for (entity, GameInteraction { interaction }) in changed.iter() {
        if let Ok(square) = squares.get_mut(entity) {
            let proposed_move = Move(square.position, active_player);
            match interaction {
                Interaction::None => {
                    despawn_ew.send(Despawn::new(entity));
                }
                Interaction::Hover => {
                    if test_move(proposed_move, all_squares.clone()) {
                        spawn_ew.send(Spawn::new(entity, active_player));
                    }
                }
                Interaction::JustReleased => {
                    move_ew.send(proposed_move);
                }
                _ => {}
            }
        }
    }
}
