use std::collections::{HashMap, HashSet};

use crate::{
    types::{Dimensions, Line, Player, Position, DIRECTIONS},
    Move,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LineTracker {
    occupied: HashSet<Position>,
    owned: HashSet<Position>,
    potential: HashSet<Position>,
    introduced: HashMap<Position, Vec<Position>>,
    player: Player,
    dimensions: Dimensions,
}

impl LineTracker {
    pub fn new(player: Player, dimensions: &Dimensions) -> Self {
        LineTracker {
            player,
            dimensions: *dimensions,
            occupied: Default::default(),
            owned: Default::default(),
            potential: Default::default(),
            introduced: Default::default(),
        }
    }

    pub fn sort(&self, moves: &mut Vec<Move>) {
        for i in 0..moves.len() {
            if self.potential.contains(&moves[i].0) {
                let temp = moves[i];
                moves[i] = moves[0];
                moves[0] = temp;
                break;
            }
        }
    }

    pub fn score(&self) -> i32 {
        100 * self.potential.difference(&self.occupied).count() as i32
    }

    pub fn apply(&mut self, m: &Move) {
        self.occupied.insert(m.0);
        if self.player != m.1 {
            return;
        }
        self.owned.insert(m.0);
        for direction in DIRECTIONS.iter() {
            // Find the furthest
            let mut opposite = Line::new(&direction.opposite(), m.0, &self.dimensions);
            let mut max = opposite.next().unwrap();
            while let Some(next) = opposite.next() {
                if self.owned.contains(&next) {
                    max = next;
                } else {
                    break;
                }
            }

            // See if there's three in a row with a single hole.
            let mut line = Line::new(&direction, max, &self.dimensions);
            let mut run = 0;
            let mut potential = None;
            while let Some(next) = line.next() {
                if self.owned.contains(&next) {
                    run += 1;
                } else if self.occupied.contains(&next) {
                    break;
                } else if potential.is_none() {
                    potential = Some(next);
                } else {
                    break;
                }
            }

            if run >= 3 {
                if let Some(empty) = potential {
                    if !self.potential.contains(&empty) {
                        self.potential.insert(empty);

                        // keep track of which position was responsible for introducing this
                        if !self.introduced.contains_key(&m.0) {
                            self.introduced.insert(m.0, Vec::new());
                        }
                        self.introduced.get_mut(&m.0).unwrap().push(empty);
                    }
                }
            }
        }
    }

    pub fn undo(&mut self, m: &Move) {
        self.occupied.remove(&m.0);
        if self.player != m.1 {
            return;
        }
        self.owned.remove(&m.0);

        if self.introduced.contains_key(&m.0) {
            for empty in self.introduced[&m.0].iter() {
                self.potential.remove(empty);
            }
            self.introduced.get_mut(&m.0).unwrap().clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        types::{Dimensions, Player, Position},
        Move,
    };

    use super::LineTracker;

    #[test]
    fn score() {
        let line_tracker = LineTracker::new(Player::Red, &Dimensions { x: 7, y: 6 });

        assert_eq!(line_tracker.score(), 0);

        let mut line_tracker = LineTracker::new(Player::Red, &Dimensions { x: 7, y: 6 });
        line_tracker.apply(&Move(Position::new(4, 0), Player::Red));

        assert_eq!(line_tracker.score(), 0);

        let mut line_tracker = LineTracker::new(Player::Red, &Dimensions { x: 7, y: 6 });
        line_tracker.apply(&Move(Position::new(4, 0), Player::Red));
        line_tracker.apply(&Move(Position::new(5, 0), Player::Red));
        assert_eq!(line_tracker.score(), 0);

        let mut line_tracker = LineTracker::new(Player::Red, &Dimensions { x: 7, y: 6 });
        line_tracker.apply(&Move(Position::new(6, 0), Player::Red));
        line_tracker.apply(&Move(Position::new(3, 0), Player::Red));
        line_tracker.apply(&Move(Position::new(5, 0), Player::Red));
        assert_eq!(line_tracker.score(), 100);

        let mut line_tracker = LineTracker::new(Player::Red, &Dimensions { x: 7, y: 6 });
        line_tracker.apply(&Move(Position::new(6, 0), Player::Red));
        line_tracker.apply(&Move(Position::new(3, 0), Player::Red));
        line_tracker.apply(&Move(Position::new(5, 0), Player::Red));
        line_tracker.undo(&Move(Position::new(5, 0), Player::Red));
        assert_eq!(line_tracker.score(), 0);

        let mut line_tracker = LineTracker::new(Player::Red, &Dimensions { x: 7, y: 6 });
        line_tracker.apply(&Move(Position::new(6, 0), Player::Red));
        line_tracker.apply(&Move(Position::new(3, 0), Player::Red));
        line_tracker.apply(&Move(Position::new(5, 0), Player::Red));
        line_tracker.undo(&Move(Position::new(5, 0), Player::Red));
        line_tracker.apply(&Move(Position::new(4, 0), Player::Red));
        assert_eq!(line_tracker.score(), 100);

        let mut line_tracker = LineTracker::new(Player::Red, &Dimensions { x: 7, y: 6 });
        line_tracker.apply(&Move(Position::new(6, 0), Player::Red));
        line_tracker.apply(&Move(Position::new(3, 0), Player::Red));
        line_tracker.apply(&Move(Position::new(5, 0), Player::Red));
        line_tracker.undo(&Move(Position::new(5, 0), Player::Red));
        line_tracker.apply(&Move(Position::new(4, 0), Player::Red));
        line_tracker.apply(&Move(Position::new(5, 0), Player::Blue));
        assert_eq!(line_tracker.score(), 0);
    }
}
