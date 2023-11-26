use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use app::AppPlugin;
use game::GamePlugin;
use main_menu::MainMenuPlugin;

pub mod app;
pub mod game;
pub mod main_menu;

fn main() {
    App::new()
        .add_plugins((AppPlugin, MainMenuPlugin, GamePlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Update, bevy::window::close_on_esc)
        .run()
}
