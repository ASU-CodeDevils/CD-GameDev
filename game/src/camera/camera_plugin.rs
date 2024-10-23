use avian2d::prelude::*;
use bevy::{input::mouse::MouseWheel, prelude::*};

pub struct CameraPlugin;

//Component that marks our Camera's current target
#[derive(Component)]
pub struct CameraTarget;

//Component that marks our main camera
#[derive(Component)]
pub struct MainCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_main_camera);
        app.add_systems(
            PostUpdate,
            update_camera
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
        );
        app.add_systems(Update, zoom_camera);
    }
}

//Sets up a 2d camera and attaches the MainCamera marker to it
fn setup_main_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

//Update camera function will have our main camera follow any entity with the CameraTarget tag
fn update_camera(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<CameraTarget>)>,
    target_query: Query<&Transform, (With<CameraTarget>, Without<MainCamera>)>,
) {
    let Ok(mut camera) = camera_query.get_single_mut() else {
        error!("could not execute query for single MainCamera component");
        return;
    };

    let Ok(target) = target_query.get_single() else {
        error!("Could not execute query for single CameraTarget component");
        return;
    };

    let Vec3 { x, y, .. } = target.translation;

    let direction = Vec3::new(x, y, camera.translation.z);

    camera.translation = direction;
}

//Camera Zooming Function, uses MouseWheel Scrolling up/down for zooming in/out
fn zoom_camera(
    mut scroll_event: EventReader<MouseWheel>,
    mut camera: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    //Zoom Sensitivity
    let zoom_factor = 0.1;

    //Amount to zoom in this update cycle
    let mut zoom_amount = 0.0;

    for event in scroll_event.read() {
        zoom_amount += -event.y * zoom_factor
    }

    if zoom_amount != 0.0 {
        for mut projection in camera.iter_mut() {
            projection.scale += zoom_amount;

            //This statement makes sure that the amount of zoom is capped, can change if need be
            projection.scale = projection.scale.clamp(0.1, 5.0);
        }
    }
}
