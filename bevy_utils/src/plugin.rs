use bevy::prelude::{App, Plugin};

macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    ($app:expr) => {
        // The macro will expand into the contents of this block.
        $app.add_startup_system(print_hello);
    };
}

fn print_hello() {
    println!("Hello!");
}

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        say_hello!(app);
    }
}
