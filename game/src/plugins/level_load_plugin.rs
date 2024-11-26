use crate::components::{
    collidable::{Collidable, CollidableBundle},
    health::{Health, HealthBundle},
    player::{Player, PlayerBundle},
};
use crate::movement::plugin::*;
use avian2d::math::{Scalar, Vector};
use avian2d::prelude::*;
use bevy::{ecs::entity, math::vec2, prelude::*};
use bevy_ecs_ldtk::prelude::*;

const LDTK_PROJECT_NAME: &str = "TestLevel.ldtk";
const PLAYER_HITBOX_SHIFT_X: f32 = 0.0;
const PLAYER_HITBOX_SHIFT_Y: f32 = -12.0;
const PLAYER_HITBOX_ROTATION: f32 = 0.0;
const PLAYER_HITBOX_WIDTH: f32 = 8.0;
const PLAYER_HITBOX_LENGTH: f32 = 24.0;
const PLAYER_ACCELERATION: Scalar = 125.0;
const PLAYER_STARTING_HEALTH: u32 = 100;
const PLAYER_DAMPING: Scalar = 1.0;
const PLAYER_JUMP_IMPULSE: Scalar = 130.0;
const PLAYER_MAX_SLOPE_ANGLE: Scalar = 10.0 as Scalar;
const PLAYER_COLLIDER_DENSITY: ColliderDensity = ColliderDensity(50000.0);
const PLAYER_GRAVITY_SCALE: GravityScale = GravityScale(10.0);
const PLAYER_COLLISION_MARGIN: CollisionMargin = CollisionMargin(5.0);
const COLLIDABLES_SHIFT_CENTER: f32 = 12.0;
const COLLIDABLES_SHIFT_VERTEX: f32 = 24.0;
const COLLIDABLES_COLLIDER_SIZE_X: f32 = 24.0;
const COLLIDABLES_COLLIDER_SIZE_Y: f32 = 24.0;
const COLLIDABLES_COLLIDER_DENSITY: ColliderDensity = ColliderDensity(100000.0);

///LevelLoadPlugin handle the loading of an LDTtk project into the game
///The plugin handles parsing in the world and entities
pub struct LevelLoadPlugin;

impl Plugin for LevelLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, world_setup)
            .insert_resource(LevelSelection::index(0))
            .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_int_cell::<CollidableBundle>(1)
            .add_systems(Update, (collidables_setup, player_setup).chain());
    }
}

///world_setup spawns a basic camera (to be replaced later)
///and parses in the LDtk project
fn world_setup(mut command: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    camera.transform.translation.x += 1280.0 / 4.0;
    camera.transform.translation.y += 720.0 / 4.0;
    command.spawn(camera);

    command.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load(LDTK_PROJECT_NAME),
        ..Default::default()
    });
}

///player_setup queries for any entities that were added the previous update
///with the "Player" tag then adds appropriate components
fn player_setup(mut commands: Commands, query: Query<(Entity, &Transform), Added<Player>>) {
    let new_players = query.iter();

    for (entity, player_position) in new_players {
        let shapes = vec![(
            Position::new(Vec2::new(PLAYER_HITBOX_SHIFT_X, PLAYER_HITBOX_SHIFT_Y)),
            Rotation::degrees(PLAYER_HITBOX_ROTATION),
            Collider::rectangle(PLAYER_HITBOX_WIDTH, PLAYER_HITBOX_LENGTH),
        )];

        let player_health = HealthBundle {
            health: Health::new(PLAYER_STARTING_HEALTH),
        };

        let new_control = CharacterControllerBundle::new(Collider::compound(shapes)).with_movement(
            PLAYER_ACCELERATION,
            PLAYER_DAMPING,
            PLAYER_JUMP_IMPULSE,
            PLAYER_MAX_SLOPE_ANGLE.to_radians(),
        );

        commands
            .entity(entity)
            .insert(PLAYER_COLLISION_MARGIN);
        commands.entity(entity).insert(player_health);
        commands.entity(entity).insert(new_control);
        commands
            .entity(entity)
            .insert(Friction::ZERO.with_combine_rule(CoefficientCombine::Min));
        commands
            .entity(entity)
            .insert(Restitution::ZERO.with_combine_rule(CoefficientCombine::Min));
        commands
            .entity(entity)
            .insert(PLAYER_COLLIDER_DENSITY);
        commands
            .entity(entity)
            .insert(PLAYER_GRAVITY_SCALE);
    }
}

///collidable_setup queries for any entities that were added the previous update
///with the "Collidable" tag then adds appropriate components
fn collidables_setup(
    mut commands: Commands,
    new_tile_query: Query<(Entity, &Transform), Added<Collidable>>,
) {
    let new_collidables = new_tile_query.iter();

    for (_new_tile, position) in new_collidables {
        let mut new_position = *position;

        new_position.translation.x += COLLIDABLES_SHIFT_CENTER;
        new_position.translation.y += COLLIDABLES_SHIFT_CENTER;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite { ..default() },
                transform: new_position,
                ..default()
            },
            RigidBody::Static,
            Collider::rectangle(COLLIDABLES_COLLIDER_SIZE_X, COLLIDABLES_COLLIDER_SIZE_Y),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            COLLIDABLES_COLLIDER_DENSITY,
        ));
    }
}
