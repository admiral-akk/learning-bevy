use bevy::prelude::{App, Plugin, SystemSet};

use super::{
    board::setup,
    events::{Move, ResetBoard},
    update::update,
};

pub struct ModelPlugin {
    pub stage: &'static str,
}

impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ResetBoard>()
            .add_event::<Move>()
            .add_system_set_to_stage(self.stage, SystemSet::new().with_system(update))
            .add_startup_system(setup);
    }
}
