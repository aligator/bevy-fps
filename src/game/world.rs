use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_world(
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
        .insert(Collider::halfspace(Vec3::Y).unwrap());

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