use std::marker::PhantomData;

use bevy::prelude::*;
use connect_4_model::{
    types::{Owner, Position},
    Model, Move, MoveResult,
};
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem};
use k_utils::{
    util_action::{handle_actions, Act},
    util_plugin::UtilPlugin,
    util_resource::add_util_resource,
    util_state::{StateContraint, UtilState},
};
use std::{fmt::Debug, hash::Hash};

use crate::main_menu::plugin::MainMenu;

use self::mut_deref::setup_board;

use derive_more::{Deref, Into};

use super::{actions::Actions, plugin::Game};

#[derive(Component, Deref, PartialEq, Clone, Copy, Hash, Eq, Debug, Into)]
pub struct PositionW(Position);

#[derive(Component, Deref, PartialEq, Clone, Copy, Hash, Eq, Debug, Into)]
pub struct OwnerW(Owner);

mod mut_deref {
    use std::ops::DerefMut;

    use crate::game::input::{Bot, Column, Human};

    use super::{Owner, OwnerW, PositionW};
    use bevy::prelude::*;
    use connect_4_model::types::{Player, Position};

    impl DerefMut for OwnerW {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    pub fn setup_board(mut commands: Commands) {
        for column in 0..7 {
            commands.spawn(Column(column));
        }
        for column in 0..7 {
            for row in 0..6 {
                commands
                    .spawn(PositionW(Position::new(column, row)))
                    .insert(OwnerW(Owner::None));
            }
        }
        commands.spawn(Human(Player::Red));
        commands.spawn(Bot(Player::Blue, None));
    }
}

fn apply_move(
    mut commands: Commands,
    mut action_ewr: EventReader<Act<Actions>>,
    mut board: Query<(&PositionW, &mut OwnerW)>,
    mut history: ResMut<MoveHistory>,
) {
    for action in action_ewr.iter() {
        match action.action {
            Actions::EndGame(s) => match s {
                k_utils::util_button::State::JustReleased(_) => {
                    Game::exit_to::<MainMenu>(&mut commands);
                    return;
                }
                _ => {}
            },
            Actions::Move(m) => {
                let model = Model::from(history.0.iter());
                let mr = model.predict(&m);
                if let MoveResult(Some(m)) = mr {
                    history.0.push(m);
                    for (pos, mut owner) in board.iter_mut() {
                        if m.0.eq(pos) {
                            owner.0 = Owner::Owned(m.1);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct Logic<StateType: StateContraint> {
    _phantom: PhantomData<StateType>,
}

#[derive(Resource)]
pub struct MoveHistory(pub Vec<Move>);

impl FromWorld for MoveHistory {
    fn from_world(_world: &mut World) -> Self {
        MoveHistory(Vec::new())
    }
}

impl<StateType: StateContraint> Plugin for Logic<StateType> {
    fn build(&self, app: &mut App) {
        app.add_enter_system(UtilState::<StateType>::Enter, setup_board);
        add_util_resource::<StateType, MoveHistory>(app);
        handle_actions::<StateType>(app, apply_move.into_conditional());
    }
}
