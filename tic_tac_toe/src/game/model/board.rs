use std::ops::RangeInclusive;

use bevy::{
    prelude::{
        Commands,
    },
};

use super::types::{Position, Square};

const MAX: usize = 2;
const RANGE: RangeInclusive<usize> = 0..=MAX;
pub fn setup(mut commands: Commands) {
    for x in RANGE {
        for y in RANGE {
            commands.spawn(Square {
                position: Position(x, y),
                owner: None,
            });
        }
    }
}
