use bevy::prelude::{EventReader, Query};

use super::{events::Move, types::Square};

pub fn update(mut moves: EventReader<Move>, mut squares: Query<&mut Square>) {
    for Move(pos, owner) in moves.iter() {
        for mut square in squares.iter_mut() {
            if square.position.eq(pos) && square.owner.is_none() {
                square.owner = Some(*owner);
                break;
            }
        }
    }
}
