use bevy::prelude::*;

use crate::components::{damage::Damage, health::Health, player::Player};

///This plugin provides terminal debug capabilities
///By adding systems, requested information will be printed to the terminal
pub struct DebugPlugin;

///add the system(s) you wish to use to have debug info printed to terminal
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, print_debug_active_message);
    }
}

///Function that verifies that the debugger is active by printing a single message to the terminal on startup
///To use, add system using add_systems
fn print_debug_active_message() {
    info!("The debugger is active!");
}
