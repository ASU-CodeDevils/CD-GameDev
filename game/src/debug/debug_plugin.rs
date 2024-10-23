use avian2d::math::*;
use avian2d::prelude::*;
use bevy::prelude::*;

use crate::camera::camera_plugin::*;
use crate::movement::plugin::*;

///This plugin provides terminal debug capabilities
///By adding systems, requested information will be printed to the terminal
pub struct DebugPlugin;

///add the system(s) you wish to use to have debug info printed to terminal
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, print_debug_active_message);
        app.add_systems(Startup, display_dudes);
    }
}

///Function that verifies that the debugger is active by printing a single message to the terminal on startup
///To use, add system using add_systems
fn print_debug_active_message() {
    info!("The debugger is active!");
}

//Displays two pixel like sprites where one is stationary, then the other that has the CameraTarget component
//falls forever. Used to test camera system tracking and zooming.
fn display_dudes(mut commands: Commands) {
    commands
        .spawn((
            SpriteBundle::default(),
            CharacterControllerBundle::new(Collider::capsule(12.5, 20.0)).with_movement(
                1250.0,
                0.92,
                400.0,
                (30.0 as Scalar).to_radians(),
            ),
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            ColliderDensity(2.0),
            GravityScale(1.5),
        ))
        .insert(CameraTarget);

    commands.spawn(SpriteBundle::default());
}
