use crate::components::health::Alive;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

///Pointer struct for Player entities
#[derive(Default, Component)]
pub struct Player;

///Bundle to be applied to Player entities
///when parsed from LDtk project
#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    #[sprite_sheet_bundle]
    pub sprite_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    pub grid_coords: GridCoords,
    pub status: Alive,
}
