use crate::line_tracker::LineTracker;
use crate::types::{Dimensions, Owner, Player};
use crate::Move;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ScoreTracker {
    pub red: LineTracker,
    pub blue: LineTracker,
}

impl ScoreTracker {
    pub fn new(dimensions: &Dimensions) -> ScoreTracker {
        ScoreTracker {
            red: LineTracker::new(Player::Red, dimensions),
            blue: LineTracker::new(Player::Blue, dimensions),
        }
    }

    pub fn sort(&self, moves: &mut Vec<Move>, player: Player) {
        // Sort forced blocking moves to the front
        match player {
            Player::Red => self.blue.sort(moves),
            Player::Blue => self.red.sort(moves),
        }
        // Sort winning moves to the front
        match player {
            Player::Red => self.red.sort(moves),
            Player::Blue => self.blue.sort(moves),
        }
    }
}

impl ScoreTracker {
    pub fn apply(&mut self, m: &Move) {
        self.red.apply(m);
        self.blue.apply(m);
    }
    pub fn undo(&mut self, m: &Move) {
        self.red.undo(m);
        self.blue.undo(m);
    }
    pub fn score(&self, active_player: Player) -> i32 {
        (self.red.score() - self.blue.score())
            * (match active_player {
                Player::Red => 1,
                Player::Blue => -1,
            })
    }
}
