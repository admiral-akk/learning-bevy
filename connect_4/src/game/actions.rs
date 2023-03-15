use connect_4_model::Move;

use k_utils::util_button::State;

#[derive(Clone, Copy)]
pub enum Actions {
    EndGame(State),
    Move(Move),
}
