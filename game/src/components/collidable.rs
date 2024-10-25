use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

///Pointer struct for collidable entities
#[derive(Default, Component)]
pub struct Collidable;

///Bundle to be applied to Collidable entities when
///parsed from the LDtk project
#[derive(Default, Bundle, LdtkIntCell)]
pub struct CollidableBundle {
    pub wall: Collidable,
}
