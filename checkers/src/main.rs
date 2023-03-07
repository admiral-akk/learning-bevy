use bevy::{prelude::App, DefaultPlugins};
use k_utils::util_plugin::{add_stages, UtilPluginStruct};

fn main() {
    let mut app = App::new();
    add_stages(&mut app);
    // Add plugins
    app.add_plugins(DefaultPlugins).add_plugin(UtilPluginStruct);

    // Run game
    app.run();
}
