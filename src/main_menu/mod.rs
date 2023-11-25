use self::systems::{
    interactions::{animate_buttons, interact_with_play_button},
    layout::{despawn_main_menu, spawn_main_menu},
};
use crate::app::AppState;
use bevy::prelude::*;
use crate::main_menu::systems::camera::{remove_camera, spawn_camera};

pub mod components;
pub mod elements;
pub mod styles;
pub mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), (spawn_camera, spawn_main_menu))
            .add_systems(
                Update,
                (animate_buttons, interact_with_play_button).run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), (despawn_main_menu, remove_camera));
    }
}
