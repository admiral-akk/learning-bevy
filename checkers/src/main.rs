use bevy::prelude::*;
use game::plugin::Game;
use k_utils::util_plugin::{add_stages, UtilPluginStruct};
use start_menu::plugin::StartMenu;

mod game;
mod start_menu;

fn main() {
    let mut app = App::new();
    add_stages(&mut app);
    // Add plugins
    app.add_plugin(UtilPluginStruct)
        .add_plugin(StartMenu)
        .add_plugin(Game);

    // Run game
    app.run();
}
