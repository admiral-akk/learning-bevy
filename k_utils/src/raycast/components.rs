use bevy::prelude::Component;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Interaction {
    #[default]
    None,
    Hover,
    JustClicked,
    Clicked,
    JustReleased,
}

#[derive(Component, Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct GameInteraction {
    pub interaction: Interaction,
}

impl GameInteraction {
    pub fn new() -> GameInteraction {
        GameInteraction {
            interaction: Interaction::None,
        }
    }
}
