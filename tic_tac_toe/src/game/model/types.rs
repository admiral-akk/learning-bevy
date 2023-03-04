use std::ops::RangeInclusive;

use bevy::prelude::Component;

use super::events::Move;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Player {
    X,
    O,
}

#[derive(Component)]
pub struct WinningLine;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position(pub usize, pub usize);

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Line(pub Vec<Position>, Option<Player>);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    pub position: Position,
    pub owner: Option<Player>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineFold {
    Nothing,
    None,
    Owner(Player),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Winner {
    None,
    Tie,
    Player(Player),
}

const MAX: usize = 2;
const RANGE: RangeInclusive<usize> = 0..=MAX;
pub struct Result(pub Winner, pub Option<Vec<Line>>);

fn fold_winners<OwnerIter: Iterator<Item = Option<Player>> + Clone>(
    owners: OwnerIter,
) -> Option<Player> {
    let winner = owners
        .into_iter()
        .fold(LineFold::Nothing, |acc, owner| match (owner, acc) {
            (None, _) => LineFold::None,
            (_, LineFold::None) => LineFold::None,
            (Some(owner), LineFold::Nothing) => LineFold::Owner(owner),
            (Some(owner), LineFold::Owner(other)) => {
                if owner == other {
                    acc
                } else {
                    LineFold::None
                }
            }
        });

    match winner {
        LineFold::Nothing => None,
        LineFold::None => None,
        LineFold::Owner(winner) => Some(winner),
    }
}

impl Line {
    fn new<Iter: Iterator<Item = Square> + Clone>(positions: Vec<Position>, squares: Iter) -> Line {
        let winner = fold_winners(
            squares
                .into_iter()
                .filter(|square| positions.contains(&square.position))
                .map(|square| square.owner),
        );

        Line(positions, winner)
    }

    fn lines<Iter: Iterator<Item = Square> + Clone>(squares: Iter) -> Vec<Line> {
        let mut lines = Vec::new();
        for x in RANGE {
            let mut line = Vec::new();
            for y in RANGE {
                line.push(Position(x, y));
            }
            lines.push(Line::new(line, squares.clone()));
        }
        for y in RANGE {
            let mut line = Vec::new();
            for x in RANGE {
                line.push(Position(x, y));
            }
            lines.push(Line::new(line, squares.clone()));
        }
        let mut line = Vec::new();
        let mut line2 = Vec::new();
        for x in RANGE {
            line.push(Position(x, x));
            line2.push(Position(x, MAX - x));
        }
        lines.push(Line::new(line, squares.clone()));
        lines.push(Line::new(line2, squares.clone()));

        lines
    }
}

pub fn test_move<Iter: Iterator<Item = Square> + Clone>(
    proposed_move: Move,
    squares: Iter,
) -> bool {
    let active_player = active_player(squares.clone());
    if active_player != proposed_move.1 {
        return false;
    }
    let winner = winner(squares.clone());
    if winner.0 != Winner::None {
        return false;
    }
    return squares
        .clone()
        .any(|square| square.position == proposed_move.0 && square.owner.is_none());
}

pub fn active_player<Iter: Iterator<Item = Square> + Clone>(squares: Iter) -> Player {
    let x_count = squares
        .clone()
        .filter(|square| square.owner == Some(Player::X))
        .count();
    let o_count = squares
        .filter(|square| square.owner == Some(Player::O))
        .count();
    if x_count > o_count {
        Player::O
    } else {
        Player::X
    }
}

pub fn winner<Iter: Iterator<Item = Square> + Clone>(squares: Iter) -> Result {
    let has_empty_square = squares.clone().any(|square| square.owner.is_none());
    let lines = Line::lines(squares);
    let winning_lines = lines.iter().filter(|line| line.1.is_some());

    let winner = fold_winners(winning_lines.clone().map(|line| line.1));

    match (winner, has_empty_square) {
        (None, true) => Result(Winner::None, None),
        (None, false) => Result(Winner::Tie, None),
        (Some(winner), _) => Result(
            Winner::Player(winner),
            Some(winning_lines.cloned().collect()),
        ),
    }
}
