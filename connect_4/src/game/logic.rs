use std::marker::PhantomData;

use bevy::prelude::*;
use connect_4_model::{
    types::{Owner, Position},
    Move,
};
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem};
use k_utils::{
    util_action::handle_actions,
    util_resource::add_util_resource,
    util_state::{StateContraint, UtilState},
};
use std::{fmt::Debug, hash::Hash};

use self::mut_deref::{apply_exit, apply_moves, setup_board, simulate_exit, simulate_moves};

use derive_more::{Deref, Into};

#[derive(Component, Deref, PartialEq, Clone, Copy, Hash, Eq, Debug, Into)]
pub struct PositionW(Position);

#[derive(Component, Deref, PartialEq, Clone, Copy, Hash, Eq, Debug, Into)]
pub struct OwnerW(Owner);

mod mut_deref {
    use std::ops::DerefMut;

    use super::{MoveHistory, Owner, OwnerW, PositionW};

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

    use k_utils::{
        util_action::{Proposal, Selection},
        util_plugin::UtilPlugin,
    };

    use crate::{
        game::{
            event::{ExitGame, ExitingGame, MoveResultW, MoveW},
            input::{Bot, Column, Human},
            plugin::Game,
        },
        main_menu::plugin::MainMenu,
    };
    use bevy::prelude::*;
    use connect_4_model::{
        types::{Player, Position},
        Model, MoveResult,
    };

    pub fn simulate_moves(
        mut commands: Commands,
        m: Query<(Entity, &MoveW), With<Proposal>>,
        move_history: Res<MoveHistory>,
    ) {
        for (entity, m) in m.iter() {
            let model = Model::from(move_history.0.iter());
            commands
                .get_entity(entity)
                .unwrap()
                .insert(MoveResultW::new(model.predict(m)));
        }
    }
    pub fn apply_moves(
        m: Query<&MoveW, With<Selection>>,
        mut board: Query<(&PositionW, &mut OwnerW)>,
        mut history: ResMut<MoveHistory>,
    ) {
        for m in m.iter() {
            let model = Model::from(history.0.iter());
            let mr = model.predict(m);
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

    pub fn simulate_exit(mut commands: Commands, m: Query<(Entity, &ExitGame), With<Proposal>>) {
        for (entity, _) in m.iter() {
            commands.get_entity(entity).unwrap().insert(ExitingGame);
        }
    }

    pub fn apply_exit(mut commands: Commands, e: Query<&ExitGame, With<Selection>>) {
        for _ in e.iter() {
            Game::exit_to::<MainMenu>(&mut commands);
            return;
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
        handle_actions::<StateType>(
            app,
            simulate_moves.into_conditional(),
            apply_moves.into_conditional(),
        );
        handle_actions::<StateType>(
            app,
            simulate_exit.into_conditional(),
            apply_exit.into_conditional(),
        );
    }
}
