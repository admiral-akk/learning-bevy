use bevy::prelude::*;

use k_utils::{util_plugin::UtilPlugin, util_state::StateContraint};

use super::{actions::Actions, ui::*};

#[derive(Debug, Default, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StartMenu;

impl Plugin for StartMenu {
    fn build(&self, app: &mut App) {
        Self::add_defaults(app);
        app.add_plugin(UI::<Self>::default());
    }
}

impl StateContraint for StartMenu {}
impl UtilPlugin<StartMenu, Actions> for StartMenu {
    fn active() -> bool {
        true
    }
}
