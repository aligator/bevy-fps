use bevy::{prelude::*};
use bevy::gltf::Gltf;

use crate::app::MyAssets;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
/// helper marker component
pub struct LoadedMarker;

pub fn spawn_world(
    mut commands: Commands,
    assets: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    scene_markers_query: Query<&LoadedMarker>,
) {
    // sky color
    commands.insert_resource(ClearColor(Color::hsl(203., 0.51, 0.51)));

    if scene_markers_query.is_empty() {
        if let Some(gltf) = assets_gltf.get(&assets.world) {
            // spawn a default scene
            commands.spawn((
                Name::new("World"),
                SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    ..default()
                },
                LoadedMarker
            ));
        }
    }

    commands.spawn(TextBundle::from_section(
        "Press `Q` key to switch angular state",
        TextStyle::default(),
    ));
}