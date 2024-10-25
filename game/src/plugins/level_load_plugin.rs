use crate::components::{
    collidable::{Collidable, CollidableBundle},
    health::{Health, HealthBundle},
    player::{Player, PlayerBundle},
};
use crate::movement::plugin::*;
use avian2d::math::{Scalar, Vector};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

const LDTK_PROJECT_NAME: &str = "TestLevel.ldtk";
const PLAYER_COLLIDER_RADIUS: Scalar = 12.5;
const PLAYER_COLLIDER_LOWER_ENDPOINT: Vector = Vector::new(0.0, -15.0);
const PLAYER_COLLIDER_UPPER_ENDPOINT: Vector = Vector::new(0.0, -8.0);
const PLAYER_ACCELERATION: Scalar = 125.0;
const PLAYER_STARTING_HEALTH: u32 = 100;
const PLAYER_DAMPING: Scalar = 1.0;
const PLAYER_JUMP_IMPULSE: Scalar = 130.0;
const PLAYER_MAX_SLOPE_ANGLE: Scalar = 30.0 as Scalar;
const PLAYER_COLLIDER_DENSITY: ColliderDensity = ColliderDensity(5.0);
const PLAYER_GRAVITY_SCALE: GravityScale = GravityScale(10.0);
const COLLIDABLES_SHIFT: f32 = 12.0;
const COLLIDABLES_COLLIDER_SIZE_X: f32 = 23.9;
const COLLIDABLES_COLLIDER_SIZE_Y: f32 = 23.9;
const COLLIDABLES_COLLIDER_DENSITY: ColliderDensity = ColliderDensity(1000.0);

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
fn player_setup(mut commands: Commands, query: Query<Entity, Added<Player>>) {
    let new_players = query.iter();

    for entity in new_players {
        let player_health = HealthBundle {
            health: Health::new(PLAYER_STARTING_HEALTH),
        };

        let new_control = CharacterControllerBundle::new(Collider::capsule_endpoints(
            PLAYER_COLLIDER_RADIUS,
            PLAYER_COLLIDER_LOWER_ENDPOINT,
            PLAYER_COLLIDER_UPPER_ENDPOINT,
        ))
        .with_movement(
            PLAYER_ACCELERATION,
            PLAYER_DAMPING,
            PLAYER_JUMP_IMPULSE,
            PLAYER_MAX_SLOPE_ANGLE.to_radians(),
        );

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
    query: Query<(Entity, &Transform), Added<Collidable>>,
) {
    let new_collidables = query.iter();

    for (_collidable, position) in new_collidables {
        let mut new_position = *position;
        new_position.translation.x += COLLIDABLES_SHIFT;
        new_position.translation.y += COLLIDABLES_SHIFT;

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
