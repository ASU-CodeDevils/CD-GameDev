#![allow(unused)]
mod animation;
mod components;
mod debug;
mod movement;
mod plugins;

use crate::animation::animation_system::SpriteAnimationPlugin;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use debug::debug_plugin::DebugPlugin;
use movement::plugin::CharacterControllerPlugin;
use plugins::health_and_damage_plugin::HealthAndDamagePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(LdtkPlugin)
        .add_plugins(CharacterControllerPlugin)
        //user plugins
        .add_plugins(HealthAndDamagePlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(SpriteAnimationPlugin)
        .run();
}
