use alpha_beta::GameModel;
use score_tracker::ScoreTracker;
use std::hash::{Hash, Hasher};
use std::{collections::hash_map::DefaultHasher, fmt::Debug};
use types::{Dimensions, Owner, Player, Position};

mod line_tracker;
mod score_tracker;
mod test;
pub mod types;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Model {
    pub squares: Vec<(usize, Vec<Owner>)>,
    pub dimensions: (usize, usize),
    pub active_player: Player,
    winner: Option<Player>,
    hash: u64,
    score_tracker: ScoreTracker,
}

impl Default for Model {
    fn default() -> Self {
        let dimensions = (7, 6);
        let d = Dimensions {
            x: dimensions.0,
            y: dimensions.1,
        };
        Self {
            dimensions,
            active_player: Player::Red,
            squares: vec![(0, vec![Owner::None; dimensions.1]); dimensions.0],
            winner: Default::default(),
            hash: Default::default(),
            score_tracker: ScoreTracker::new(&d),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Hash, Eq, Debug)]
pub struct Move(pub Position, pub Player);

#[derive(PartialEq, Clone, Copy, Hash, Eq, Debug)]
pub struct MoveResult(pub Option<Move>);

impl Hash for Model {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl GameModel for Model {
    type GameMove = Move;
    fn legal_moves(&self) -> Vec<Move> {
        match self.winner {
            Some(_) => Vec::new(),
            None => {
                let mut moves = (0..self.dimensions.0)
                    .filter_map(|x| match self.next_y(x) {
                        Some(y) => Some(Move(Position { x, y }, self.active_player)),
                        None => None,
                    })
                    .collect::<Vec<_>>();
                self.score_tracker.sort(&mut moves, self.active_player);
                moves
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let mr = self.predict(m);
        match mr.0 {
            Some(Move(pos, player)) => {
                let mut hasher = DefaultHasher::new();
                pos.hash(&mut hasher);
                player.hash(&mut hasher);
                self.hash ^= hasher.finish();
                self.squares[pos.x].0 += 1;
                self.squares[pos.x].1[pos.y] = Owner::Owned(player);
                self.update_winner(&pos);
                self.score_tracker.apply(m);
                self.active_player = self.active_player.opponent();
            }
            None => {
                println!("Illegal move! Move: {:?}", m);
                println!("Illegal move! Board: {:?}", self.squares);
            }
        }
    }

    fn undo(&mut self, m: &Move) {
        let mut hasher = DefaultHasher::new();
        let pos = m.0;
        pos.hash(&mut hasher);
        m.1.hash(&mut hasher);
        self.winner = None;
        self.hash ^= hasher.finish();
        self.squares[pos.x].1[pos.y] = Owner::None;
        self.squares[pos.x].0 -= 1;
        self.score_tracker.undo(m);
        self.active_player = self.active_player.opponent();
    }

    fn score(&self) -> i32 {
        match self.winner {
            None => self.score_tracker.score(self.active_player),
            Some(player) => {
                if player == self.active_player {
                    10000
                } else {
                    -10000
                }
            }
        }
    }
}

impl<'a, It: Iterator<Item = &'a Move>> From<It> for Model {
    fn from(value: It) -> Self {
        let mut model = Model::default();
        for m in value {
            model.apply(&m);
        }
        model
    }
}

impl Model {
    pub fn predict(&self, m: &Move) -> MoveResult {
        if m.1 != self.active_player {
            return MoveResult(None);
        }
        if self.squares[m.0.x].1[m.0.y] != Owner::None {
            return MoveResult(None);
        }
        if self.squares[m.0.x].0 != m.0.y {
            return MoveResult(None);
        }
        return MoveResult(Some(m.clone()));
    }

    fn update_winner(&mut self, pos: &Position) {
        let kernel = pos.kernel(self.dimensions);
        for line in kernel {
            match line.owner(&self) {
                Owner::Owned(player) => {
                    self.winner = Some(player);
                    return;
                }
                Owner::None => {}
            }
        }
    }

    fn next_y(&self, x: usize) -> Option<usize> {
        let y = self.squares[x].0;
        if y == self.dimensions.1 {
            None
        } else {
            Some(y)
        }
    }
}

struct Line(pub Vec<Position>);
impl Line {
    pub fn new() -> Line {
        Line(Vec::new())
    }

    pub fn owner(&self, model: &Model) -> Owner {
        let mut run = 0;
        let mut line_owner = Owner::None;
        for pos in self.0.iter() {
            let owner = model.squares[pos.x].1[pos.y];
            if line_owner == owner {
                run += 1;
            } else {
                line_owner = owner;
                run = 1;
            }
            if run >= 4 {
                return line_owner;
            }
        }
        Owner::None
    }

    pub fn almost_owner(&self, squares: &Vec<(usize, Vec<Owner>)>) -> Vec<(Position, Player)> {
        let mut v = self
            .0
            .iter()
            .map(|&pos| (pos, squares[pos.x].1[pos.y], 0))
            .collect::<Vec<_>>();
        for i in 0..v.len() {
            if i == 0 {
                v[i].2 = 1;
            } else {
                if v[i].1 == v[i - 1].1 {
                    v[i].2 = v[i - 1].2 + 1;
                } else {
                    v[i].2 = 1;
                }
            }
        }
        for i in (0..v.len()).rev() {
            if i < v.len() - 1 {
                if v[i].1 == v[i + 1].1 {
                    v[i].2 = v[i + 1].2;
                }
            }
        }

        let mut almost = Vec::new();
        for (i, &val) in v.iter().enumerate() {
            if val.1 != Owner::None {
                continue;
            }
            if i > 0 && i < v.len() - 2 {
                if v[i - 1].1 == v[i + 1].1 {
                    if let Owner::Owned(player) = v[i - 1].1 {
                        if v[i - 1].2 + v[i + 1].2 >= 3 {
                            almost.push((v[i - 1].0, player));
                        }
                    }
                } else {
                    if let Owner::Owned(player) = v[i - 1].1 {
                        if v[i - 1].2 >= 3 {
                            almost.push((v[i - 1].0, player));
                        }
                    }
                    if let Owner::Owned(player) = v[i + 1].1 {
                        if v[i + 1].2 >= 3 {
                            almost.push((v[i - 1].0, player));
                        }
                    }
                }
            }
        }
        almost
    }
}

impl Position {
    fn maybe_position(x: i32, y: i32, dimensions: (usize, usize)) -> Option<Position> {
        if x >= 0 && y >= 0 {
            let (x, y) = (x as usize, y as usize);
            if x < dimensions.0 && y < dimensions.1 {
                return Some(Position { x, y });
            }
        }
        None
    }

    // If any 3 in a row are the same colour, then playing self will complete 4 in row
    fn kernel(&self, dimensions: (usize, usize)) -> Vec<Line> {
        let (x, y) = (self.x as i32, self.y as i32);
        let mut v = vec![Line::new(), Line::new(), Line::new(), Line::new()];

        for i in -3_i32..=3 {
            match Position::maybe_position(x + i, y, dimensions) {
                Some(pos) => {
                    v[0].0.push(pos);
                }
                _ => {}
            }
            match Position::maybe_position(x, y + i, dimensions) {
                Some(pos) => {
                    v[1].0.push(pos);
                }
                _ => {}
            }
            match Position::maybe_position(x + i, y + i, dimensions) {
                Some(pos) => {
                    v[2].0.push(pos);
                }
                _ => {}
            }
            match Position::maybe_position(x + i, y - i, dimensions) {
                Some(pos) => {
                    v[3].0.push(pos);
                }
                _ => {}
            }
        }

        v
    }
}

#[cfg(test)]
mod tests {
    use alpha_beta::GameModel;

    use crate::{
        types::{Owner, Player, Position},
        Model, Move,
    };

    #[test]
    fn default_model() {
        let model = Model::default();

        assert_eq!(model.winner, None);
        assert_eq!(model.active_player, Player::Red);
        assert_eq!(model.dimensions, (7, 6));
        assert_eq!(model.squares.len(), model.dimensions.0);
        assert!(model
            .squares
            .iter()
            .all(|column| column.1.len() == model.dimensions.1));
        assert!(model.squares.iter().all(|column| column.0 == 0));
        assert!(model
            .squares
            .iter()
            .all(|column| column.1.iter().all(|&owner| owner == Owner::None)),)
    }

    #[test]
    fn legal_moves_alter_state() {
        let base_model = Model::default();
        let moves = base_model.legal_moves();
        for m in moves.iter() {
            let mut model = Model::default();
            model.apply(&m);
            assert_ne!(model, base_model);
        }
    }

    #[test]
    fn apply_undo_maintains_state() {
        let base_model = Model::default();
        let moves = base_model.legal_moves();
        for m in moves.iter() {
            let mut model = Model::default();
            model.apply(&m);
            model.undo(&m);
            assert_eq!(model, base_model);
        }
    }

    #[test]
    fn model_from_moves() {
        let moves = [
            Move(Position::new(0, 0), Player::Red),
            Move(Position::new(4, 0), Player::Blue),
            Move(Position::new(4, 1), Player::Red),
        ];
        let model = Model::from(moves.iter());
        assert_eq!(model.winner, None);
        assert_eq!(model.active_player, Player::Blue);
        assert_eq!(model.dimensions, (7, 6));
        assert_eq!(model.squares.len(), model.dimensions.0);
        assert!(model
            .squares
            .iter()
            .all(|column| column.1.len() == model.dimensions.1));
        assert_eq!(model.squares[0].0, 1);
        assert_eq!(model.squares[4].0, 2);
        assert_eq!(model.squares[0].1[0], Owner::Owned(Player::Red));
        assert_eq!(model.squares[4].1[0], Owner::Owned(Player::Blue));
        assert_eq!(model.squares[4].1[1], Owner::Owned(Player::Red));
        assert!(model
            .squares
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != 0 && i != 4)
            .all(|(_, column)| column.0 == 0));
        assert!(model
            .squares
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != 0 && i != 4)
            .all(|(_, column)| column.1.iter().all(|&owner| owner == Owner::None)));
    }

    #[test]
    fn winner() {
        // Vertical
        let moves = [
            Move(Position::new(0, 0), Player::Red),
            Move(Position::new(1, 0), Player::Blue),
            Move(Position::new(0, 1), Player::Red),
            Move(Position::new(1, 1), Player::Blue),
            Move(Position::new(0, 2), Player::Red),
            Move(Position::new(1, 2), Player::Blue),
            Move(Position::new(0, 3), Player::Red),
        ];
        let model = Model::from(moves.iter());
        assert_eq!(model.winner, Some(Player::Red));
        assert!(model.legal_moves().is_empty());

        // Horizontal
        let moves = [
            Move(Position::new(0, 0), Player::Red),
            Move(Position::new(0, 1), Player::Blue),
            Move(Position::new(1, 0), Player::Red),
            Move(Position::new(1, 1), Player::Blue),
            Move(Position::new(2, 0), Player::Red),
            Move(Position::new(2, 1), Player::Blue),
            Move(Position::new(3, 0), Player::Red),
        ];
        let model = Model::from(moves.iter());
        assert_eq!(model.winner, Some(Player::Red));
        assert!(model.legal_moves().is_empty());
        // Diagonal up-left
        let moves = [
            Move(Position::new(1, 0), Player::Red),
            Move(Position::new(3, 0), Player::Blue),
            Move(Position::new(1, 1), Player::Red),
            Move(Position::new(1, 2), Player::Blue),
            Move(Position::new(2, 0), Player::Red),
            Move(Position::new(2, 1), Player::Blue),
            Move(Position::new(0, 0), Player::Red),
            Move(Position::new(0, 1), Player::Blue),
            Move(Position::new(0, 2), Player::Red),
            Move(Position::new(0, 3), Player::Blue),
        ];
        let model = Model::from(moves.iter());
        assert_eq!(model.winner, Some(Player::Blue));
        assert!(model.legal_moves().is_empty());

        // Diagonal up-right
        let moves = [
            Move(Position::new(2, 0), Player::Red),
            Move(Position::new(0, 0), Player::Blue),
            Move(Position::new(2, 1), Player::Red),
            Move(Position::new(2, 2), Player::Blue),
            Move(Position::new(1, 0), Player::Red),
            Move(Position::new(1, 1), Player::Blue),
            Move(Position::new(3, 0), Player::Red),
            Move(Position::new(3, 1), Player::Blue),
            Move(Position::new(3, 2), Player::Red),
            Move(Position::new(3, 3), Player::Blue),
        ];
        let model = Model::from(moves.iter());
        assert_eq!(model.winner, Some(Player::Blue));
        assert!(model.legal_moves().is_empty());
    }
    #[test]
    fn score_vertical() {
        let moves = [];
        let model = Model::from(moves.iter());
        assert_eq!(model.score(), 0);

        let moves = [Move(Position::new(0, 0), Player::Red)];
        let model = Model::from(moves.iter());
        assert_eq!(model.score(), 0);

        let moves = [
            Move(Position::new(0, 0), Player::Red),
            Move(Position::new(1, 0), Player::Blue),
            Move(Position::new(0, 1), Player::Red),
        ];
        let model = Model::from(moves.iter());
        assert_eq!(model.score(), 0);

        let moves = [
            Move(Position::new(0, 0), Player::Red),
            Move(Position::new(1, 0), Player::Blue),
            Move(Position::new(0, 1), Player::Red),
            Move(Position::new(1, 1), Player::Blue),
            Move(Position::new(0, 2), Player::Red),
        ];
        let model = Model::from(moves.iter());
        assert_eq!(model.score(), -100);
        let moves = [
            Move(Position::new(0, 0), Player::Red),
            Move(Position::new(1, 0), Player::Blue),
            Move(Position::new(0, 1), Player::Red),
            Move(Position::new(1, 1), Player::Blue),
            Move(Position::new(0, 2), Player::Red),
            Move(Position::new(1, 2), Player::Blue),
        ];
        let model = Model::from(moves.iter());
        assert_eq!(model.score(), 0);

        let moves = [
            Move(Position::new(0, 0), Player::Red),
            Move(Position::new(1, 0), Player::Blue),
            Move(Position::new(0, 1), Player::Red),
            Move(Position::new(1, 1), Player::Blue),
            Move(Position::new(0, 2), Player::Red),
            Move(Position::new(1, 2), Player::Blue),
            Move(Position::new(0, 3), Player::Red),
        ];
        let model = Model::from(moves.iter());
        assert_eq!(model.score(), -10000);

        let moves = [
            Move(Position::new(0, 0), Player::Red),
            Move(Position::new(1, 0), Player::Blue),
            Move(Position::new(0, 1), Player::Red),
            Move(Position::new(1, 1), Player::Blue),
            Move(Position::new(0, 2), Player::Red),
            Move(Position::new(1, 2), Player::Blue),
            Move(Position::new(2, 0), Player::Red),
            Move(Position::new(0, 3), Player::Blue),
        ];
        let model = Model::from(moves.iter());
        assert_eq!(model.score(), -100);
    }
    #[test]
    fn score_horizontal() {
        let moves = [];
        let model = Model::from(moves.iter());
        assert_eq!(model.score(), 0);

        let moves = [Move(Position::new(0, 0), Player::Red)];
        let model = Model::from(moves.iter());
        assert_eq!(model.score(), 0);

        let moves = [
            Move(Position::new(0, 0), Player::Red),
            Move(Position::new(0, 1), Player::Blue),
            Move(Position::new(1, 0), Player::Red),
        ];
        let model = Model::from(moves.iter());
        assert_eq!(model.score(), 0);

        let moves = [
            Move(Position::new(0, 0), Player::Red),
            Move(Position::new(0, 1), Player::Blue),
            Move(Position::new(1, 0), Player::Red),
            Move(Position::new(1, 1), Player::Blue),
            Move(Position::new(2, 0), Player::Red),
        ];
        let model = Model::from(moves.iter());
        assert_eq!(model.score(), -100);
        let moves = [
            Move(Position::new(0, 0), Player::Red),
            Move(Position::new(0, 1), Player::Blue),
            Move(Position::new(1, 0), Player::Red),
            Move(Position::new(1, 1), Player::Blue),
            Move(Position::new(2, 0), Player::Red),
            Move(Position::new(2, 1), Player::Blue),
        ];
        let model = Model::from(moves.iter());
        assert_eq!(model.score(), 0);

        let moves = [
            Move(Position::new(0, 0), Player::Red),
            Move(Position::new(0, 1), Player::Blue),
            Move(Position::new(1, 0), Player::Red),
            Move(Position::new(1, 1), Player::Blue),
            Move(Position::new(2, 0), Player::Red),
            Move(Position::new(2, 1), Player::Blue),
            Move(Position::new(3, 0), Player::Red),
        ];
        let model = Model::from(moves.iter());
        assert_eq!(model.score(), -10000);

        let moves = [
            Move(Position::new(0, 0), Player::Red),
            Move(Position::new(0, 1), Player::Blue),
            Move(Position::new(1, 0), Player::Red),
            Move(Position::new(1, 1), Player::Blue),
            Move(Position::new(2, 0), Player::Red),
            Move(Position::new(2, 1), Player::Blue),
            Move(Position::new(4, 0), Player::Red),
            Move(Position::new(3, 0), Player::Blue),
        ];
        let model = Model::from(moves.iter());
        assert_eq!(model.score(), -100);
    }
}
