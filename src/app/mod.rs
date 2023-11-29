use bevy::{prelude::*, window::close_on_esc};
use bevy::gltf::Gltf;
use bevy_asset_loader::prelude::*;
use bevy_editor_pls::controls;
use bevy_editor_pls::controls::EditorControls;
use bevy_editor_pls::prelude::*;
use bevy_gltf_components::ComponentsFromGltfPlugin;

use crate::app::camera::CameraPlugin;
use crate::app::lightning::LightingPlugin;
use crate::app::physics::PhysicsPlugin;

mod camera;
mod lightning;
mod physics;

pub struct AppPlugin;

#[derive(States, Clone, Copy, Eq, PartialEq, Debug, Hash, Default)]
pub enum AppState {
    #[default]
    AssetLoading,
    MainMenu,
    Game,
    GameOver,
}

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
            app.add_state::<AppState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1024.0, 768.0).into(),
                title: "bevy-fps".to_string(),
                ..default()
            }),
            ..default()
        }))
            .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
            .add_loading_state(
                LoadingState::new(AppState::AssetLoading)
                    .continue_to_state(AppState::MainMenu)
            )
            .add_collection_to_loading_state::<_, MyAssets>(AppState::AssetLoading)

            .insert_resource(editor_controls())
            .add_plugins(EditorPlugin::default())

            // Gltf import
            .add_plugins((
                ComponentsFromGltfPlugin,
            ))

            // Core plugins
            .add_plugins((LightingPlugin, CameraPlugin, PhysicsPlugin))
            .add_systems(Update, close_on_esc);
    }
}

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    #[asset(path = "models/World.glb")]
    pub world: Handle<Gltf>,
}

fn editor_controls() -> EditorControls {
    let mut editor_controls = EditorControls::default_bindings();
    editor_controls.unbind(controls::Action::PlayPauseEditor);

    editor_controls.insert(
        controls::Action::PlayPauseEditor,
        controls::Binding {
            input: controls::UserInput::Single(controls::Button::Keyboard(KeyCode::F1)),
            conditions: vec![controls::BindingCondition::ListeningForText(false)],
        },
    );

    editor_controls
}
