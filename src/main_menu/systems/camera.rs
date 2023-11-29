use bevy::prelude::*;

use crate::main_menu::components::MainMenuCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("MenuCamera"),
        MainMenuCamera {},
        Camera2dBundle {
            ..Default::default()
        }));
}

pub fn remove_camera(mut commands: Commands, query: Query<Entity, With<MainMenuCamera>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}