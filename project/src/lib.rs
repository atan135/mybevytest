use bevy::prelude::*;

mod game;

#[bevy_main]
pub fn main() {
    run();
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(game::GamePlugin)
        .run();
}
