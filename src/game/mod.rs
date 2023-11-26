use std::f32::consts::FRAC_2_PI;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_rapier3d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_rapier3d::{TnuaRapier3dIOBundle, TnuaRapier3dPlugin};

use crate::app::AppState;

mod world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins((TnuaRapier3dPlugin, TnuaControllerPlugin))
            .add_systems(OnEnter(AppState::Game), (world::spawn_world, spawn_player))
            .add_systems(Update, (
                toggle_cursor_lock,
                apply_controls.before(execute_move),
                apply_mouse.before(execute_move),
                execute_move
            ).run_if(in_state(AppState::Game)));
    }
}

#[derive(Component, Default)]
pub struct PlayerBody {
    desired_rotation: Quat,
    desired_velocity: Vec3,
}

#[derive(Component)]
pub struct PlayerCamera {}


fn toggle_cursor_lock(
    input: Res<Input<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if input.just_pressed(KeyCode::Tab) {
        let mut window = windows.single_mut();
        match window.cursor.grab_mode {
            CursorGrabMode::Locked => {
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;
            }
            _ => {
                window.cursor.grab_mode = CursorGrabMode::Locked;
                window.cursor.visible = false;
            }
        }
    }
}

fn spawn_player(mut commands: Commands) {
    let mut cmd = commands.spawn_empty();

    // Insert the player mesh
    cmd.insert((
        TransformBundle::from_transform(Transform::from_xyz(0.0, 0.2, 0.0)),
        PlayerBody::default()
    )).with_children(|builder| {
        // Attach the camera to the player
        builder.spawn((
            PlayerCamera {},
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.7, 0.0),
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

    // cmd.insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z);
}

fn apply_controls(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut PlayerBody, &Transform)>,
) {
    for (mut body, transform) in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::W) {
            direction += transform.forward();
        }
        if keyboard.pressed(KeyCode::S) {
            direction += transform.back();
        }
        if keyboard.pressed(KeyCode::A) {
            direction += transform.left()
        }
        if keyboard.pressed(KeyCode::D) {
            direction += transform.right()
        }

        direction = direction.clamp_length_max(1.0);

        let speed_factor = 2.0;

        body.desired_velocity = direction * speed_factor;
    }
}

fn apply_mouse(
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
    mut player_query: Query<(&mut PlayerBody, &Transform), Without<PlayerCamera>>,
    mut input: EventReader<MouseMotion>,
) {
    let mut camera_transform = camera_query.single_mut();
    let mut mouse_move: Vec2 = -(input.read().map(|motion| &motion.delta).sum::<Vec2>());

    for (mut body, body_transform) in player_query.iter_mut() {
        // Vertical
        let rot = camera_transform.rotation;

        // Ensure the vertical rotation is clamped
        if rot.x > FRAC_2_PI && mouse_move.y.is_sign_positive()
            || rot.x < -FRAC_2_PI && mouse_move.y.is_sign_negative()
        {
            mouse_move.y = 0.0;
        }

        camera_transform.rotate(Quat::from_scaled_axis(
            rot * Vec3::X * mouse_move.y / 180.0,
        ));

        // Horizontal
        let rot = body_transform.rotation;

        let mut new_rotation = body_transform.clone();
        new_rotation.rotate(Quat::from_scaled_axis(
            rot * Vec3::Y * mouse_move.x / 180.0,
        ));

        body.desired_rotation = new_rotation.rotation;
    }
    /*
    // Vertical
    let rot = cam_tf.rotation;

    // Ensure the vertical rotation is clamped
    if rot.x > FRAC_2_PI && cumulative.y.is_sign_positive()
        || rot.x < -FRAC_2_PI && cumulative.y.is_sign_negative()
    {
        cumulative.y = 0.0;
    }

    cam_tf.rotate(Quat::from_scaled_axis(
        rot * Vec3::X * cumulative.y / 180.0,
    ));

    // Horizontal
    let rot = body_tf.rotation;
    body_tf.rotate(Quat::from_scaled_axis(
        rot * Vec3::Y * cumulative.x / 180.0,
    ));*/
}

fn execute_move(mut player_query: Query<(&mut TnuaController, &mut Transform, &PlayerBody)>) {
    for (mut controller, mut transform, body) in player_query.iter_mut() {
        controller.basis(TnuaBuiltinWalk {
            desired_velocity: body.desired_velocity,
            desired_forward: Vec3::ZERO,
            float_height: 0.2,
            ..default()
        });
        transform.rotation = body.desired_rotation;
    }
}