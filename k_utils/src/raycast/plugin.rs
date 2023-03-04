use bevy::prelude::{App, IntoSystemDescriptor, Plugin, SystemSet};

use super::{
    game_interaction::handle_mouse,
    mouse::{entity_removed, mouse_system, UiInteraction},
    raycast_2d::{deregister_sprite, register_sprite, update_sprite, Raycaster2d},
};

pub struct RaycastPlugin {
    pub pre_input_stage: &'static str,
    pub update_changed_stage: &'static str,
}

impl Plugin for RaycastPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Raycaster2d::new())
            .insert_resource(UiInteraction::new())
            .add_system_set_to_stage(
                self.pre_input_stage,
                SystemSet::new()
                    .with_system(update_sprite)
                    .with_system(mouse_system)
                    .with_system(handle_mouse.after(mouse_system).after(update_sprite)),
            )
            .add_system_set_to_stage(
                self.update_changed_stage,
                SystemSet::new()
                    .with_system(register_sprite)
                    .with_system(deregister_sprite)
                    .with_system(entity_removed),
            );
    }
}
