use connect_4_model::Move;
use iyes_loopless::{condition::ConditionalSystemDescriptor, prelude::IntoConditionalSystem};
use k_utils::{util_action::Action, util_button::State};
use std::marker::PhantomData;

use bevy::prelude::*;
use connect_4_model::{
    types::{Owner, Position},
    Model, MoveResult,
};
use iyes_loopless::prelude::AppLooplessStateExt;
use k_utils::{
    util_action::handle_actions,
    util_plugin::UtilPlugin,
    util_resource::add_util_resource,
    util_state::{StateContraint, UtilState},
};
use std::{fmt::Debug, hash::Hash};

use crate::main_menu::plugin::MainMenu;

use derive_more::{Deref, Into};

use super::{
    logic::{MoveHistory, OwnerW, PositionW},
    plugin::Game,
};
#[derive(Clone, Copy)]
pub enum Actions {
    EndGame(State),
    Move(Move),
}
