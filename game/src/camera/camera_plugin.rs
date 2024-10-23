use bevy::{input::mouse::MouseWheel, prelude::*};

pub struct CameraPlugin;

//Component that marks our Camera's current target
#[derive(Component)]
pub struct CameraTarget;

//Component that marks our main camera
#[derive(Component)]
pub struct MainCamera;

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, setup_main_camera);
        app.add_systems(Update, (update_camera, zoom_camera).chain());
    }
}


//Sets up a 2d camera and attaches the MainCamera marker to it 
fn setup_main_camera(
    mut commands: Commands
) {
    commands.spawn(( 
        Camera2dBundle::default(),
        MainCamera    
    ));
}


//Update camera function will have our main camera follow any entity with the CameraTarget tag
fn update_camera(
    mut camera_query : Query<&mut Transform, (With<MainCamera>, Without<CameraTarget>)>,
    target_query: Query<&Transform, (With<CameraTarget>, Without<MainCamera>)>,
){
    //getting camera from query
    let Ok(mut camera) = camera_query.get_single_mut() else{
        return;
    };

    //getting target from query
    let Ok(target) = target_query.get_single() else{
        return;
    };

    //Gets the x and y coordinates of our target
    let Vec3 {x,y,..} = target.translation;

    //Stores the target x y with our camera z cuz our camera z not changing
    let direction = Vec3::new(x,y,camera.translation.z);

    //Debug statements if we want to use them :)
    //info!("Camera: {:?}", camera);
    //info!("Target: {:?}", target);

    //Make our camera translation match our target translation
    camera.translation = direction;
}

//Camera Zooming Function, uses MouseWheel Scrolling up/down for zooming in/out
fn zoom_camera(
    mut scroll_event : EventReader<MouseWheel>,
    mut camera: Query<&mut OrthographicProjection, With<MainCamera>>
){


    //Zoom Sensitivity
    let zoom_factor = 0.1;

    //Amount to zoom in this update cycle
    let mut zoom_amount = 0.0;


    //For every event read in this cycle increase the zoom amount by 1 event * sensitivity
    for event in scroll_event.read(){
        zoom_amount += -event.y * zoom_factor
    }

    //If we have had any mouse scroll events in this update then apply it to the camera projection
    if zoom_amount != 0.0{
        //extracting the type of projection from camera, only ever going to be Orthographic since 2d but I don't know a better way lol
        for mut projection in camera.iter_mut(){
            projection.scale += zoom_amount;

            //This statement makes sure that the amount of zoom is capped, can change if need be 
            projection.scale = projection.scale.clamp(0.1,5.0);
        }
    }
}
