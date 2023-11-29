use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_tnua::control_helpers::TnuaCrouchEnforcerPlugin;
use bevy_tnua::prelude::*;
use bevy_tnua_rapier3d::TnuaRapier3dPlugin;

use crate::app::AppState;
use crate::game::player::{apply_controls, apply_mouse, execute_move, spawn_player, toggle_cursor_lock};

mod world;
mod player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())

            .add_plugins((TnuaRapier3dPlugin, TnuaControllerPlugin, TnuaCrouchEnforcerPlugin))
            .add_systems(OnEnter(AppState::Game), (world::spawn_world, spawn_player))
            .add_systems(Update, (
                toggle_cursor_lock,
                apply_controls.before(execute_move),
                apply_mouse.before(execute_move),
                execute_move.in_set(TnuaUserControlsSystemSet),
            ).run_if(in_state(AppState::Game)));
    }
}

