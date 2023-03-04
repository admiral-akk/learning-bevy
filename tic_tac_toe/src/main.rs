use bevy::{prelude::*, utils::Instant};
use game::plugin::GamePlugin;
use iyes_loopless::prelude::AppLooplessStateExt;
use raycast::plugin::RaycastPlugin;
mod game;
mod raycast;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    InGame,
}
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_loopless_state(GameState::MainMenu)
        .insert_resource(Time::new(Instant::now()))
        .add_plugin(RaycastPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(setup_camera);

    app.run();
}

fn setup_camera(mut commands: Commands, mut windows: ResMut<Windows>) {
    commands.spawn(Camera2dBundle::default());
    let window = windows.primary_mut();
    window.set_resolution(900., 900.);
    window.set_resizable(false);
}
