
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    time::{Duration, SystemTime},
};

mod test;

#[derive(Default)]
struct TranspositionTable {
    table: HashMap<u64, i32>,
}

#[derive(Default)]
struct PVSTable<Move> {
    table: HashMap<u64, Move>,
}

impl<Move: PartialEq + Copy> PVSTable<Move> {
    fn sort(&self, board: &impl Hash, moves: &mut Vec<Move>) {
        let mut hasher = DefaultHasher::new();
        board.hash(&mut hasher);
        let hash = hasher.finish();
        if let Some(m) = self.table.get(&hash) {
            if let Some(index) = moves.iter().position(|r| r == m) {
                moves[index] = moves[0];
                moves[0] = *m;
            }
        }
    }

    fn get(&self, board: &impl Hash) -> Option<Move> {
        let mut hasher = DefaultHasher::new();
        board.hash(&mut hasher);
        let hash = hasher.finish();
        self.table.get(&hash).copied()
    }

    fn add_val(&mut self, board: &impl Hash, m: Move) {
        let mut hasher = DefaultHasher::new();
        board.hash(&mut hasher);
        let hash = hasher.finish();
        self.table.insert(hash, m);
    }
}

impl TranspositionTable {
    fn depth_hash(board: &impl Hash, depth: usize) -> u64 {
        let mut hasher = DefaultHasher::new();
        board.hash(&mut hasher);
        depth.hash(&mut hasher);
        hasher.finish()
    }

    fn score(&self, board: &impl Hash, depth: usize) -> Option<i32> {
        self.table
            .get(&TranspositionTable::depth_hash(board, depth))
            .copied()
    }

    fn add_val(&mut self, board: &impl Hash, depth: usize, score: i32) {
        self.table
            .insert(TranspositionTable::depth_hash(board, depth), score);
    }
}

#[derive(Debug)]
pub struct AlphaBetaResult<Move> {
    pub depth: usize,
    pub score: i32,
    pub m: Move,
}

type MoveIndex = usize;
pub struct AlphaBetaState<Move> {
    pub moves: Vec<Move>,
    pub alpha: i32,
    pub beta: i32,
    pub best_move: Option<Move>,
}

impl<Move> AlphaBetaState<Move> {
    pub fn new(moves: Vec<Move>, alpha: i32, beta: i32) -> AlphaBetaState<Move> {
        AlphaBetaState {
            moves,
            alpha,
            beta,
            best_move: None,
        }
    }
}

#[derive(Default)]
pub struct AlphaBetaSearch<Move> {
    pub complete: bool,
    target_depth: usize,
    pub best_so_far: Option<AlphaBetaResult<Move>>,
    tt: TranspositionTable,
    pvs: PVSTable<Move>,
    current_depth: usize,
    state: Vec<AlphaBetaState<Move>>,
}

impl<Move: PartialEq + Copy> AlphaBetaSearch<Move> {
    fn new(target_depth: usize) -> AlphaBetaSearch<Move> {
        AlphaBetaSearch {
            complete: false,
            target_depth,
            best_so_far: None,
            tt: TranspositionTable::default(),
            pvs: PVSTable {
                table: HashMap::new(),
            },
            current_depth: 2,
            state: Vec::default(),
        }
    }

    pub fn start_search(
        model: &mut impl GameModel<GameMove = Move>,
        target_depth: usize,
        budget: Duration,
    ) -> AlphaBetaSearch<Move> {
        let mut search_data = AlphaBetaSearch::new(target_depth);
        search_data.search(model, SystemTime::now(), budget);
        search_data
    }

    pub fn continue_search(
        &mut self,
        model: &mut impl GameModel<GameMove = Move>,
        budget: Duration,
    ) {
        self.search(model, SystemTime::now(), budget);
    }

    fn generate_moves(&mut self, model: &mut impl GameModel<GameMove = Move>) -> Vec<Move> {
        let mut moves = model.legal_moves();
        self.pvs.sort(model, &mut moves);
        moves.reverse();
        moves
    }

    fn search(
        &mut self,
        model: &mut impl GameModel<GameMove = Move>,
        start: SystemTime,
        budget: Duration,
    ) {
        if self.complete {
            return;
        }
        while self.current_depth <= self.target_depth {
            if self.state.is_empty() {
                let new_state =
                    AlphaBetaState::new(self.generate_moves(model), -i32::MAX, i32::MAX);
                self.state.push(new_state);
            }

            match self.search_internal(model, start, budget, 0) {
                Ok(_) => {
                    println!("Solved depth: {}", self.current_depth);
                    self.best_so_far = Some(AlphaBetaResult {
                        depth: self.current_depth,
                        score: self.tt.score(&model, self.current_depth).unwrap(),
                        m: self.pvs.get(&model).unwrap(),
                    });
                    self.current_depth += 1;
                }
                Err(_) => {
                    return;
                }
            }
        }
        if self.current_depth > self.target_depth {
            self.complete = true;
        }
    }

    fn search_internal(
        &mut self,
        model: &mut impl GameModel<GameMove = Move>,
        start: SystemTime,
        budget: Duration,
        depth: usize,
    ) -> Result<i32, bool> {
        if depth == self.current_depth {
            return Ok(model.score());
        }
        if let Some(score) = self.tt.score(model, self.current_depth - depth) {
            return Ok(score);
        }

        // If we don't already have a state object, generate one.
        if self.state.len() <= depth {
            let moves = self.generate_moves(model);
            if moves.is_empty() {
                return Ok(model.score());
            }

            let new_state = AlphaBetaState::new(
                moves,
                -self.state[depth - 1].beta,
                -self.state[depth - 1].alpha,
            );
            self.state.push(new_state);
        }

        while !self.state[depth].moves.is_empty() {
            // If we've expended our time budget, then exit.
            if SystemTime::now().duration_since(start).unwrap() > budget {
                println!("Budget exceeded!");
                return Err(false);
            }

            let m = self.state[depth].moves.pop().unwrap();

            // Apply the move and get the score.
            model.apply(&m);
            let score = self.search_internal(model, start, budget, depth + 1);
            model.undo(&m);

            match score {
                // If we successfully found the score for this move, then move on to the next move.
                Ok(score) => {
                    if score >= self.state[depth].beta {
                        let beta = self.state[depth].beta;
                        self.state.pop();
                        return Ok(beta);
                    }
                    if score > self.state[depth].alpha {
                        self.state[depth].alpha = score;
                        self.state[depth].best_move = Some(m);
                    }
                }

                // If we timed out, then add the move back to be scored and push the error down.
                Err(v) => {
                    self.state[depth].moves.push(m);
                    return Err(v);
                }
            }
        }

        self.tt
            .add_val(model, self.current_depth - depth, self.state[depth].alpha);
        if let Some(m) = self.state[depth].best_move {
            self.pvs.add_val(model, m);
        }
        let alpha = self.state[depth].alpha;
        self.state.pop();
        Ok(alpha)
    }
}

pub trait GameModel: Hash + Sized {
    type GameMove: PartialEq + Copy;
    fn legal_moves(&self) -> Vec<Self::GameMove>;
    fn apply(&mut self, m: &Self::GameMove);
    fn undo(&mut self, m: &Self::GameMove);

    // From the perspective of the player who just moved.
    fn score(&self) -> i32;

    fn search(&mut self, depth: usize) -> Option<Self::GameMove> {
        let mut tt = TranspositionTable::default();
        let mut pvs = PVSTable {
            table: HashMap::new(),
        };
        let legal_moves = self.legal_moves();

        // Seeding pvs
        for i in 1..depth {
            let mut best = -i32::MAX;
            for m in legal_moves.iter() {
                self.apply(m);
                let score = -search_internal(self, i, &mut tt, &mut pvs, best, i32::MAX);
                if score > best {
                    best = score;
                }
                self.undo(m);
            }
        }

        let mut move_scores = Vec::new();
        let mut best = -i32::MAX;
        let mut best_so_far = None;
        for m in legal_moves {
            self.apply(&m);
            let score = -search_internal(self, depth, &mut tt, &mut pvs, best, i32::MAX);
            move_scores.push((m.clone(), score));
            if score > best {
                best = score;
                best_so_far = Some(m);
            }
            self.undo(&m);
        }
        best_so_far
        // let b: Vec<Self::GameMove> = move_scores
        //     .into_iter()
        //     .filter_map(|(m, s)| if s == best { Some(m) } else { None })
        //     .collect();
        // println!("expected score: {:?}", best);
        // b.choose(&mut rand::thread_rng()).cloned()
    }
}

// Returns the best score that the active player can get.
fn search_internal<Model: GameModel>(
    model: &mut Model,
    depth: usize,
    tt: &mut TranspositionTable,
    pvs: &mut PVSTable<Model::GameMove>,
    mut alpha: i32,
    beta: i32,
) -> i32 {
    if depth == 0 {
        return model.score();
    }
    let mut moves = model.legal_moves();
    if moves.is_empty() {
        return model.score();
    }
    match tt.score(model, depth) {
        Some(score) => {
            return score;
        }
        _ => {}
    }

    pvs.sort(model, &mut moves);

    let mut best_move = None;
    for m in moves.iter() {
        model.apply(m);
        let score = -search_internal(model, depth - 1, tt, pvs, -beta, -alpha);
        if score >= beta {
            model.undo(m);
            return beta;
        }
        if score > alpha {
            alpha = score;
            best_move = Some(*m);
        }
        model.undo(m);
    }
    tt.add_val(model, depth, alpha);
    if let Some(m) = best_move {
        pvs.add_val(model, m);
    }
    alpha
}
