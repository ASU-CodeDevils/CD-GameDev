# Tools
Tools we'll be using to develop the game.

## Level designer
[LDtk](https://ldtk.io/download/)\
Seems like a great, open source, level editor. Also integrates nicely with Bevy using the [Bevy ECS LDtk](https://docs.rs/bevy_ecs_ldtk/latest/bevy_ecs_ldtk/) crate.

# Approved 3rd party crates 
Collection of crates that are approved for use in the game code. If you want to use a crate that is not listed here, please discuss with us first. The goal is to limit the already large dependencies that we rely on to try to keep compilation times as short as possible.

## Physics
[Avian 2D](https://docs.rs/avian2d/latest/avian2d/)\
We will use Avian 2D as the physics engine behind our game. It is integrated tightly with Bevy, which should make things easier. At the moment, we are using the Avian 2D dynamic character controller example plugin, which should make interactions between physics object simpler for the time being.

## Lighting
[Bevy light 2D](https://docs.rs/bevy_light_2d/latest/bevy_light_2d/)\
Seems like a simple 2D lighting crate. Should serve our need for the time being.

[Magic light 2D](https://docs.rs/crate/bevy_magic_light_2d/latest)\
SDF based global illumination system with occlusion support.

## Particle effects
[Bevy Hanabi](https://docs.rs/bevy_hanabi/latest/bevy_hanabi/)\
The most widely used crate for GPU particle systems.

## Tilemap handling
[Bevy ECS LDtk](https://docs.rs/bevy_ecs_ldtk/latest/bevy_ecs_ldtk/)\
Allows using LDtk project files.

[Bevy ECS tilemap](https://docs.rs/bevy_ecs_tilemap/latest/bevy_ecs_tilemap/)\
A popular crate for handling tilemaps, but might not be necessary due to us using LDtk.



