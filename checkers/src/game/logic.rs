use bevy::prelude::{App, Changed, Commands, Component, EventReader, Plugin, Query, Res, Resource};
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem};
use k_utils::{
    raycast::components::GameInteraction, util_action::handle_actions, util_plugin::UtilPlugin,
    util_state::UtilState,
};

use crate::start_menu::plugin::StartMenu;

use super::{actions::Actions, plugin::Game};

#[derive(Component, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

#[derive(Resource, Debug)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

impl Default for Dimensions {
    fn default() -> Self {
        Self {
            width: 8,
            height: 8,
        }
    }
}

#[derive(Component)]
pub struct Square;

#[derive(Component)]
pub struct Piece {
    pub owner: Owner,
}

pub enum Owner {
    White,
    Black,
}

pub fn setup_board(mut commands: Commands, dimensions: Res<Dimensions>) {
    for column in 0..dimensions.height {
        for row in 0..dimensions.width {
            commands
                .spawn(Position::new(row, column))
                .insert(Square)
                .insert(GameInteraction::new());
        }
    }
    for row in 0..dimensions.width {
        commands
            .spawn(Position::new(row, row % 2))
            .insert(Piece {
                owner: Owner::White,
            })
            .insert(GameInteraction::new());
    }
    for row in 0..dimensions.width {
        commands
            .spawn(Position::new(row, 7 - (row + 1) % 2))
            .insert(Piece {
                owner: Owner::Black,
            })
            .insert(GameInteraction::new());
    }
}

fn detect_hover(square: Query<(&Position, &GameInteraction), Changed<GameInteraction>>) {
    for s in square.iter() {
        println!("{:?}", s);
    }
}

fn apply_move(mut commands: Commands, mut action_ewr: EventReader<Actions>) {
    for action in action_ewr.iter() {
        match action {
            Actions::ExitGame(s) => match s {
                k_utils::util_button::State::JustReleased(_) => {
                    Game::exit_to::<StartMenu>(&mut commands);
                    return;
                }
                _ => {}
            },
        }
    }
}

#[derive(Default)]
pub struct Logic {}

impl Plugin for Logic {
    fn build(&self, app: &mut App) {
        app.insert_resource(Dimensions::default());
        app.add_enter_system(UtilState::<Game>::Enter, setup_board);
        handle_actions::<Game>(app, apply_move.into_conditional());
        handle_actions::<Game>(app, detect_hover.into_conditional());
    }
}
