use bevy::prelude::*;

use crate::utils::{util_plugin::UtilPlugin, util_state::StateContraint};

use super::{input::Input, logic::*, ui::*};

#[derive(Component)]
struct EnterGameButton;

#[derive(Debug, Default, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        Self::add_defaults(app);
        app.add_plugin(Logic::<Self>::default());
        app.add_plugin(UI::<Self>::default());
        app.add_plugin(Input::<Self>::default());
    }
}

impl StateContraint for MainMenu {}
impl UtilPlugin<MainMenu> for MainMenu {}
