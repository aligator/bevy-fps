use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_rapier3d::{TnuaRapier3dIOBundle, TnuaRapier3dPlugin};

use crate::app::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins((TnuaRapier3dPlugin, TnuaControllerPlugin))
            .add_systems(OnEnter(AppState::Game), (spawn_world, spawn_player));
        //  .add_systems(Update, (hold_on_ground).run_if(in_state(AppState::Game)))
    }
}

fn spawn_world(
    mut commands: Commands, assets_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // sky color
    commands.insert_resource(ClearColor(Color::hsl(203., 0.51, 0.51)));

    // ground
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(shape::Plane::from_size(100.).into()),
            material: materials.add(Color::rgb(0.8, 0.655, 0.317).into()),
            ..Default::default()
        })
        .insert(Collider::cuboid(100., 0., 100.));

    // map model
    commands
        .spawn(SpatialBundle {
            transform: Transform::from_xyz(0., 0., -8.),
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(SceneBundle {
                scene: assets_server.load("temple_ruins-deyama.glb#Scene0"),
                ..Default::default()
            });

            builder
                .spawn(Collider::compound(vec![
                    // floor colliders
                    (Vec3::ZERO, Quat::default(), Collider::cuboid(2., 0.3, 2.)),
                    (Vec3::ZERO, Quat::default(), Collider::cuboid(1.7, 0.6, 1.7)),
                    // stair collider
                    (
                        Vec3::new(0., 0., 2.0),
                        Quat::default(),
                        Collider::cuboid(0.5, 0.15, 0.15),
                    ),
                    (
                        Vec3::new(0., 0.3, 1.7),
                        Quat::default(),
                        Collider::cuboid(0.5, 0.15, 0.15),
                    ),
                ]))
                .insert(TransformBundle::default());
        });

    // lights
    commands.insert_resource(AmbientLight {
        brightness: 0.8,
        ..Default::default()
    });
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 100000.0 / 2.,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform {
            translation: Vec3::new(1., 2., 0.),
            rotation: Quat::from_rotation_x(-PI / 2.),
            ..Default::default()
        });

    commands.spawn(TextBundle::from_section(
        "Press `tab` key to switch angular state",
        TextStyle::default(),
    ));
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct PlayerCamera {}

fn spawn_player(mut commands: Commands) {
    let mut cmd = commands.spawn_empty();

    // Insert the player mesh
    cmd.insert((
        TransformBundle::from_transform(Transform::from_xyz(0.0, 1.0, 0.0)),
        Player {}
    )).with_children(|builder| {
        // Attach the camera to the player
        builder.spawn((
            PlayerCamera {},
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.5, 1.0),
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: 90.0 * (std::f32::consts::PI / 180.0),
                    aspect_ratio: 1.0,
                    near: 0.1,
                    far: 1000.0,
                }),
                ..default()
            }));
    });

    // Add physics to the player
    cmd.insert(RigidBody::Dynamic);
    cmd.insert(Collider::capsule_y(0.1, 0.1));
    cmd.insert(TnuaRapier3dIOBundle::default());
    cmd.insert(TnuaControllerBundle::default());
}
