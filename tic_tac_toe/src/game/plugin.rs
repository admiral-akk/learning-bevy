use bevy::prelude::{App, CoreStage, Plugin, SystemStage};

use super::{graphics::plugin::GraphicsPlugin, model::plugin::ModelPlugin};

pub struct GamePlugin;

static INPUT: &str = "input";
static GAME: &str = "game";
static OUTPUT: &str = "output";

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(CoreStage::Update, INPUT, SystemStage::parallel())
            .add_stage_after(INPUT, GAME, SystemStage::parallel())
            .add_stage_after(GAME, OUTPUT, SystemStage::parallel())
            .add_plugin(GraphicsPlugin { stage: OUTPUT })
            .add_plugin(ModelPlugin { stage: GAME });
    }
}
