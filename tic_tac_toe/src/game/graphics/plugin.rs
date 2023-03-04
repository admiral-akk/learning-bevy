use bevy::{
    prelude::{App, IntoSystemDescriptor, Plugin, SystemSet},
    sprite::Material2dPlugin,
    time::Time,
};

use super::{
    board::setup,
    brush::BrushMaterial,
    events::{Animate, Despawn, Spawn},
    spawn::{animate, check_win, clear_lines, despawn, spawn, square_updated, update_time},
    square::{added_square, square_reset, square_update},
    ui::{button_system, spawn_ui},
};
pub struct GraphicsPlugin {
    pub stage: &'static str,
}

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<BrushMaterial>::default())
            .add_event::<Spawn>()
            .add_event::<Despawn>()
            .add_event::<Animate>()
            .insert_resource(Time::default())
            .add_startup_system(spawn_ui)
            .add_startup_system(setup)
            .add_system(button_system)
            .add_system_set_to_stage(
                self.stage,
                SystemSet::new()
                    .with_system(update_time)
                    .with_system(spawn)
                    .with_system(despawn.before(spawn))
                    .with_system(animate.after(spawn))
                    .with_system(square_update)
                    .with_system(square_reset)
                    .with_system(added_square)
                    .with_system(square_updated)
                    .with_system(check_win)
                    .with_system(clear_lines),
            );
    }
}
