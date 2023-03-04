use bevy::{
    prelude::{Entity, Input, MouseButton, Query, Res, Vec2},
    window::Windows,
};

use super::{
    components::{GameInteraction, Interaction},
    mouse::UiInteraction,
    raycast_2d::Raycaster2d,
};

fn get_hovered(
    mouse: Res<UiInteraction>,
    raycaster: Res<Raycaster2d>,
    windows: Res<Windows>,
) -> Vec<Entity> {
    if mouse.captured() {
        return Vec::new();
    }
    let window = windows.get_primary().unwrap();
    let cursor_pos = window.cursor_position();
    if cursor_pos.is_none() {
        return Vec::new();
    }
    let cursor_pos = cursor_pos.unwrap();

    raycaster.raycast(Vec2::new(
        cursor_pos.x - window.width() / 2.,
        cursor_pos.y - window.height() / 2.,
    ))
}

pub fn handle_mouse(
    mut interactable: Query<(Entity, &mut GameInteraction)>,
    mouse: Res<UiInteraction>,
    raycaster: Res<Raycaster2d>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    let hovered_entities = get_hovered(mouse, raycaster, windows);
    for (e, mut interactable) in interactable.iter_mut() {
        let next = if hovered_entities.contains(&e) {
            if mouse_input.just_released(MouseButton::Left) {
                Interaction::JustReleased
            } else if mouse_input.just_pressed(MouseButton::Left) {
                Interaction::JustClicked
            } else if mouse_input.pressed(MouseButton::Left) {
                Interaction::Clicked
            } else {
                Interaction::Hover
            }
        } else {
            Interaction::None
        };
        if interactable.interaction != next {
            interactable.interaction = next;
        }
    }
}
