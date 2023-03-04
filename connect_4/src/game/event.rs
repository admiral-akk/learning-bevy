use bevy::prelude::Component;
use connect_4_model::{Move, MoveResult};
use derive_more::Deref;
#[derive(Component, Clone, Copy, Debug)]
pub struct ExitGame;

#[derive(Component, Clone, Copy, Debug)]
pub struct ExitingGame;

#[derive(Component, Deref, PartialEq, Clone, Copy, Hash, Eq, Debug)]
pub struct MoveW(Move);
impl MoveW {
    pub fn new(m: Move) -> MoveW {
        MoveW(m)
    }
}

#[derive(Component, Deref, PartialEq, Clone, Copy, Hash, Eq, Debug)]
pub struct MoveResultW(MoveResult);

impl MoveResultW {
    pub fn new(mr: MoveResult) -> MoveResultW {
        MoveResultW(mr)
    }
}
