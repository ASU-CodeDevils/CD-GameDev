mod movement;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use movement::plugin::CharacterControllerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(LdtkPlugin)
        .add_plugins(CharacterControllerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut c: Commands,
    a: Res<AssetServer>,
) {
    c.spawn(SpriteBundle {
        texture: a.load("../assets/test.png"),
        ..Default::default()
    });
}
