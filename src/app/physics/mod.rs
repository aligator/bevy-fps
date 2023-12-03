use bevy::prelude::*;
use bevy_rapier3d::prelude::ColliderMassProperties;

pub use physics_replace_proxies::*;

pub mod physics_replace_proxies;
pub mod utils;

pub mod controls;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<AutoAABBCollider>()
            .register_type::<physics_replace_proxies::Collider>()
            .register_type::<ColliderMassProperties>()

            // find a way to make serde's stuff serializable
            // .register_type::<bevy_rapier3d::dynamics::CoefficientCombineRule>()
            //bevy_rapier3d::dynamics::CoefficientCombineRule

            .add_systems(Update, physics_replace_proxies)
        //.add_system(pause_physics.in_schedule(OnEnter(GameState::InMenu)))
        //.add_system(resume_physics.in_schedule(OnEnter(GameState::InGame)))
        ;
    }
}