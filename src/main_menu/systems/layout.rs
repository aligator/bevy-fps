use bevy::prelude::*;

use crate::main_menu::{
    components::PlayButton,
    elements::{new_button, new_text_label},
    styles::get_button_text_style,
};

use super::super::{components::MainMenu, styles::MAIN_MENU_STYLE};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
           Name::new("MainMenu"),
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn((new_button(), PlayButton {}))
                .with_children(|parent| {
                    parent.spawn(new_text_label("Play!", get_button_text_style(asset_server)));
                });
        })
        .id();

    main_menu_entity
}
