use crate::core::plugin::Core;
use bevy::prelude::*;
use game::plugin::Game;
use k_utils::util_plugin::{add_stages, UtilPluginStruct};
use main_menu::plugin::MainMenu;

pub mod core;
pub mod game;
pub mod main_menu;

fn main() {
    let mut app = App::new();
    add_stages(&mut app);
    // Add plugins
    app.add_plugins(DefaultPlugins)
        .add_plugin(UtilPluginStruct)
        .add_plugin(MainMenu)
        .add_plugin(Game)
        .add_plugin(Core);

    // Run game
    app.run();
}
