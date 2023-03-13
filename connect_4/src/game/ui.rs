use std::marker::PhantomData;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use k_utils::{
    util_button::{add_button, UtilButtonConfig},
    util_graphics::update_graphics,
    util_state::{StateContraint, UtilState},
};

use super::{actions::Actions, plugin::Game};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn update_ui(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<Game>),
    >,
) {
    for (&interaction, mut color) in &mut interaction_query {
        *color = match interaction {
            Interaction::Clicked => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        }
    }
}

pub struct ExitGame;
fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let commands = &mut commands;
    let root = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                ..default()
            },
            ..Default::default()
        })
        .id();
    let button = add_button::<Actions>(
        commands,
        &asset_server,
        UtilButtonConfig {
            size: Size::new(Val::Px(300.0), Val::Px(65.0)),
            text: "Main Menu".to_string(),
        },
        Box::new(|s| Actions::EndGame(*s)),
    );

    commands.entity(root).add_child(button);
}

#[derive(Default)]
pub struct UI<StateType: StateContraint> {
    _phantom: PhantomData<StateType>,
}

impl<StateType: StateContraint> Plugin for UI<StateType> {
    fn build(&self, app: &mut App) {
        app.add_enter_system(UtilState::<StateType>::Enter, spawn_ui);
        update_graphics::<StateType>(app, vec![update_ui.into_conditional()]);
    }
}
