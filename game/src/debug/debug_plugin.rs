use bevy::prelude::*;

pub struct DebugPlugin;

//add the system(s) you wish to use to see debug info
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_test);
    }
}

//Function to test if debugger is added correctly
fn print_test() {
    info!("The debugger is working!");
}
