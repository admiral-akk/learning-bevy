use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    main_menu::plugin::MainMenu,
    utils::{
        util_plugin::UtilPlugin,
        util_state::{StateContraint, UtilState},
    },
};

#[derive(Debug, Default, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Core;
type State = UtilState<Core>;

impl Plugin for Core {
    fn build(&self, app: &mut App) {
        Self::add_defaults(app);
        app.add_enter_system(State::Uninitialized, Core::enter_system);
        app.add_enter_system(State::Enter, setup_camera);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(State::Running)
                .with_system(Self::start_game)
                .into(),
        );
    }
}

impl Core {
    fn start_game(mut commands: Commands) {
        Self::pause_to::<MainMenu>(&mut commands);
    }
}

impl UtilPlugin<Core> for Core {}
impl StateContraint for Core {}

fn setup_camera(mut commands: Commands, mut windows: ResMut<Windows>) {
    commands.spawn(Camera2dBundle::default());
    let window = windows.primary_mut();
    window.set_resolution(900., 900.);
    window.set_resizable(false);
}
