mod debug;
mod movement;
mod camera;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use debug::debug_plugin::DebugPlugin;
use movement::plugin::CharacterControllerPlugin;
use camera::camera_plugin::CameraPlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(LdtkPlugin)
        .add_plugins(CharacterControllerPlugin)
        //user plugins
        .add_plugins(DebugPlugin)
        .add_plugins(CameraPlugin)
        .run();
}

