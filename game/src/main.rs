#![allow(dead_code)]
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

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(LdtkPlugin)
        .add_plugins(CharacterControllerPlugin)
        //user plugins
        .add_plugins(HealthAndDamagePlugin)
        .add_plugins(DebugPlugin)
        //Delete me
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .run();
}

fn setup(mut command: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    camera.transform.translation.x += 1280.0 / 4.0;
    camera.transform.translation.y += 720.0 / 4.0;
    command.spawn(camera);

    command.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("TestLevel.ldtk"),
        ..Default::default()
    });

    
}
