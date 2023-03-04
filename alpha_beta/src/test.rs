use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::GameModel;

#[derive(Hash, Clone, Copy, Eq, PartialEq)]
struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Hash, Clone, Copy, Eq, PartialEq)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Position {
    fn next(&self, dir: Direction) -> Option<Position> {
        match dir {
            Direction::N => {
                if self.y < 7 {
                    Some(Position {
                        x: self.x,
                        y: self.y + 1,
                    })
                } else {
                    None
                }
            }
            Direction::NE => {
                if self.y < 7 && self.x < 7 {
                    Some(Position {
                        x: self.x + 1,
                        y: self.y + 1,
                    })
                } else {
                    None
                }
            }
            Direction::E => {
                if self.x < 7 {
                    Some(Position {
                        x: self.x + 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
            Direction::SE => {
                if self.y > 0 && self.x < 7 {
                    Some(Position {
                        x: self.x + 1,
                        y: self.y - 1,
                    })
                } else {
                    None
                }
            }
            Direction::S => {
                if self.y > 0 {
                    Some(Position {
                        x: self.x,
                        y: self.y - 1,
                    })
                } else {
                    None
                }
            }
            Direction::SW => {
                if self.y > 0 && self.x > 0 {
                    Some(Position {
                        x: self.x - 1,
                        y: self.y - 1,
                    })
                } else {
                    None
                }
            }
            Direction::W => {
                if self.x > 0 {
                    Some(Position {
                        x: self.x - 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
            Direction::NW => {
                if self.y < 7 && self.x > 0 {
                    Some(Position {
                        x: self.x - 1,
                        y: self.y + 1,
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn line(&self, dir: Direction) -> Vec<Position> {
        let mut line = Vec::new();
        let mut next = self.next(dir);
        while let Some(next_pos) = next {
            line.push(next_pos);
            next = next_pos.next(dir);
        }
        line
    }

    pub fn surround(&self) -> Vec<Position> {
        let mut surround = Vec::new();
        for e in [
            Direction::N,
            Direction::NE,
            Direction::E,
            Direction::SE,
            Direction::S,
            Direction::SW,
            Direction::W,
            Direction::NW,
        ] {
            if let Some(next) = self.next(e) {
                surround.push(next);
            }
        }
        surround
    }
}

#[derive(Hash, Clone, Copy, Eq, PartialEq)]
enum Player {
    White,
    Black,
}

impl Player {
    pub fn score(&self) -> i32 {
        match self {
            Player::White => 1,
            Player::Black => -1,
        }
    }
}
#[derive(Hash, Clone, Copy, Eq, PartialEq)]
enum Piece {
    King,
    Queen,
    Rook,
}

impl Piece {
    pub fn score(&self) -> i32 {
        match self {
            Piece::King => 3,
            Piece::Queen => 9,
            Piece::Rook => 5,
        }
    }
}
#[derive(Hash, Clone, Copy, Eq, PartialEq)]
struct GamePiece(pub Player, pub Piece);

impl GamePiece {
    pub fn score(&self) -> i32 {
        self.0.score() * self.1.score()
    }
}

#[derive(Clone, PartialEq, Copy)]
struct Move {
    pub start: Position,
    pub end: Position,
    pub piece: GamePiece,
    pub capture: Option<GamePiece>,
}

struct BadChess {
    pub board: HashMap<Position, GamePiece>,
    pub active: Player,
}

impl Hash for BadChess {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for kv in self.board.iter() {
            kv.hash(state);
        }
        self.active.hash(state);
    }
}

impl GameModel for BadChess {
    type GameMove = Move;

    fn legal_moves(&self) -> Vec<Self::GameMove> {
        let moves = Vec::new();
        for (_pos, gp) in self.board.iter().filter(|(_pos, gp)| gp.0 == self.active) {
            match gp.1 {
                Piece::King => todo!(),
                Piece::Queen => todo!(),
                Piece::Rook => todo!(),
            }
        }
        moves
    }

    fn apply(&mut self, _m: &Self::GameMove) {}

    fn undo(&mut self, _m: &Self::GameMove) {
        todo!()
    }

    fn score(&self) -> i32 {
        let mut score = 0;
        let mut has_king = HashSet::<Player>::new();
        for (_, v) in self.board.iter() {
            score += v.score();
            match v.1 {
                Piece::King => {
                    has_king.insert(v.0);
                }
                _ => {}
            }
        }

        for player in has_king {
            score += 1000 * player.score();
        }

        -self.active.score() * score
    }
}
