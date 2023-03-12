use bevy::prelude::*;

use k_utils::{util_plugin::UtilPlugin, util_state::StateContraint};

use super::{actions::Actions, logic::*, ui::*};

#[derive(Component)]
struct EnterGameButton;

#[derive(Debug, Default, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        Self::add_defaults(app);
        app.add_plugin(Logic::<Self>::default());
        app.add_plugin(UI::<Self>::default());
    }
}

impl StateContraint for MainMenu {}
impl UtilPlugin<MainMenu, Actions> for MainMenu {}
