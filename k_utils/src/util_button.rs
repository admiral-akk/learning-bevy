use std::marker::PhantomData;

use bevy::{
    prelude::{
        App, AssetServer, BuildChildren, ButtonBundle, Changed, Color, Commands, Component, Entity,
        EventWriter, Query, Res, TextBundle,
    },
    text::TextStyle,
    time::Time,
    ui::{AlignItems, BackgroundColor, Interaction, JustifyContent, Size, Style},
    utils::default,
};

use crate::util_action::{Act, Proposal};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct UtilButtonConfig {
    pub size: Size,
    pub text: String,
}

pub trait ButtonAction<ActionType>: Component + Sized {
    fn action(state: State) -> ActionType;
}

#[derive(PartialEq, Copy, Clone)]
pub enum State {
    None,
    Hover(f32),
    Exited,
    Clicked(f32),
    JustReleased(f32),
}

#[derive(Component)]
pub struct UtilButtonState {
    state: State,
}

pub fn update_buttons(
    mut buttons: Query<(&mut UtilButtonState, &mut BackgroundColor, &Interaction)>,
    time: Res<Time>,
) {
    for (mut state, mut color, interaction) in buttons.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                state.state = match state.state {
                    State::Clicked(duration) => State::Clicked(duration + time.delta_seconds()),
                    _ => State::Clicked(0.0),
                }
            }
            Interaction::Hovered => {
                state.state = match state.state {
                    State::Hover(duration) => State::Hover(duration + time.delta_seconds()),
                    State::Clicked(duration) => State::JustReleased(duration),
                    _ => State::Hover(0.0),
                }
            }
            Interaction::None => match state.state {
                State::Exited => state.state = State::None,
                State::None => {}
                _ => state.state = State::Exited,
            },
        }
        match state.state {
            State::None => *color = NORMAL_BUTTON.into(),
            State::Hover(_) => *color = HOVERED_BUTTON.into(),
            State::Exited => *color = NORMAL_BUTTON.into(),
            State::Clicked(_) => *color = PRESSED_BUTTON.into(),
            State::JustReleased(_) => *color = PRESSED_BUTTON.into(),
        }
    }
}

pub fn emit_proposal<ActionType: Send + Sync>(
    buttons: Query<(&UtilButtonState, &UtilButton<ActionType>), Changed<UtilButtonState>>,
    mut proposal: EventWriter<Proposal<ActionType>>,
) {
    proposal.send_batch(
        buttons
            .iter()
            .map(|(state, button)| (button.generate_action)(&state.state)),
    );
}

// We need an internal button event to maintain the state of the button (graphics, responsiveness)

#[derive(Component)]
pub struct UtilButton<ActionType: Sync + Send + 'static> {
    generate_action: Box<dyn Fn(&State) -> Proposal<ActionType> + Send + Sync>,
}

pub fn add_button<ActionType: Sync + Send + 'static>(
    commands: &mut Commands,
    asset_server: &AssetServer,
    config: UtilButtonConfig,
    generate_action: Box<dyn Fn(&State) -> Proposal<ActionType> + Send + Sync>,
) -> Entity {
    let text = commands
        .spawn(TextBundle::from_section(
            config.text,
            TextStyle {
                font: asset_server.load("fonts\\FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ))
        .id();

    commands
        .spawn(ButtonBundle {
            style: Style {
                size: config.size,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .insert(UtilButtonState { state: State::None })
        .insert(UtilButton::<ActionType> { generate_action })
        .add_child(text)
        .id()
}
