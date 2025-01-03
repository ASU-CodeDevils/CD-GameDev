#![allow(unused)]
mod components;
mod debug;
mod movement;
mod plugins;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use debug::debug_plugin::DebugPlugin;
use movement::plugin::CharacterControllerPlugin;
use plugins::health_and_damage_plugin::HealthAndDamagePlugin;
use plugins::level_load_plugin::LevelLoadPlugin;
use sprite_animator::SpriteAnimationPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PhysicsPlugins::default().with_length_unit(24.0))
        .add_plugins(LdtkPlugin)
        .add_plugins(CharacterControllerPlugin)
        //user plugins
        .add_plugins(HealthAndDamagePlugin)
        .add_plugins(LevelLoadPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(SpriteAnimationPlugin)
        .run();
}
