use core::fmt;
use std::{thread::current, time::Duration};

use bevy::{
    ecs::event::EventIteratorWithId,
    prelude::*,
    utils::{info, HashMap},
};

#[derive(Debug)]
pub enum SpriteAnimError {
    NoFrames(String),
    DoesNotExist(String),
}

impl fmt::Display for SpriteAnimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoFrames(msg) => write!(
                f,
                "The frames vector for animation \"{}\" cannot be empty",
                msg
            ),
            Self::DoesNotExist(msg) => write!(
                f,
                "The animation \"{}\" does not exist. Playing default animation instead.",
                msg
            ),
        }
    }
}

/// Repeating: Animation plays continuously.
/// Once: Animation plays once.
/// Mirror: Animation plays continuously forward and backwards.
#[derive(Debug, Clone)]
pub enum SpriteAnimMode {
    Repeating,
    Once,
    Mirror,
}

/// Holds the data for a single animation.
///
/// Example usage:
/// ```Rust
/// let idle_anim = SpriteAnimData::new("Idle", vec![0,1,2,3]);
/// let attack_anim = SpriteAnimData::new("Attack", vec![4,5,6,7]).with_mode(SpriteAnimMode::Once);
/// ```
#[derive(Debug, Clone)]
pub struct SpriteAnimData {
    name: String,
    frames: Vec<usize>,
    index: usize,
    mode: SpriteAnimMode,
    forward: bool,
}

impl Default for SpriteAnimData {
    fn default() -> Self {
        Self {
            name: "Default".into(),
            frames: vec![0],
            index: 0,
            mode: SpriteAnimMode::Once,
            forward: true,
        }
    }
}

impl SpriteAnimData {
    /// Creates a new `SpriteAnimData` struct and defaults to `SpriteAnimMode::Repeating`.
    pub fn new(name: String, frames: Vec<usize>) -> Self {
        Self {
            name,
            frames,
            mode: SpriteAnimMode::Repeating,
            ..default()
        }
    }

    /// Changes the mode of the animation on instantiation
    pub fn with_mode(mut self, mode: SpriteAnimMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the index of the current animation to zero and effectively starts it over
    pub fn reset(&mut self) {
        self.index = 0;
    }

    /// Gets the frame that should be played, and advances the index to the next frame.
    /// Returns an error if the `frames` vector is empty.
    fn get_frame(&mut self) -> Result<&usize, SpriteAnimError> {
        if self.frames.is_empty() {
            return Err(SpriteAnimError::NoFrames(self.name.clone()));
        }
        if self.frames.len() == 1 {
            return Ok(self.frames.first().unwrap_or(&0));
        }
        let frame = self.frames.get(self.index).unwrap();
        match self.mode {
            SpriteAnimMode::Repeating => {
                if self.index < self.frames.len() - 1 {
                    self.index += 1;
                } else {
                    self.index = 0;
                }
            }
            SpriteAnimMode::Once => {
                if self.index < self.frames.len() - 1 {
                    self.index += 1;
                }
            }
            SpriteAnimMode::Mirror => {
                if self.index == 0 {
                    self.forward = true;
                } else if self.index == self.frames.len() - 1 {
                    self.forward = false;
                }
                if self.forward {
                    self.index += 1;
                } else {
                    self.index = self.index.saturating_sub(1);
                }
            }
        }
        Ok(frame)
    }
}

/// Sprite animation controller used to play animations from a sprite sheet.
///
/// Example usage:
/// ```Rust
/// // Creation
/// let idle_anim = SpriteAnimData::new("Idle".to_string(), vec![1, 2, 3]).with_mode(SpriteAnimMode::Repeating);
/// let ac = SpriteAnimController::new().with_anim(idle_anim);
///
/// // Changing animation through systems
/// fn change_anim(mut anim_controllers: Query<&mut SpriteAnimController>) {
///     for mut ac in anim_controllers.iter_mut() {
///         ac.set_current_animation("Idle").ok();
///     }
/// }
///
/// // Changing framerate through systems
/// fn change_anim(mut anim_controllers: Query<&mut SpriteAnimController>) {
///     for mut ac in anim_controllers.iter_mut() {
///         ac.set_fps(4.0);
///     }
/// }
///
/// // Resetting an animation through systems
/// fn change_anim(mut anim_controllers: Query<&mut SpriteAnimController>) {
///     for mut ac in anim_controllers.iter_mut() {
///         ac.current.reset();
///     }
/// }
/// ```
#[derive(Component, Debug)]
pub struct SpriteAnimController {
    collection: HashMap<String, SpriteAnimData>,
    pub current: SpriteAnimData,
    timer: Timer,
    fps: f32,
}

impl SpriteAnimController {
    /// Creates a new animation controller with a "Default" animation in its collection.
    /// This default animation will always return 0 as the frame to display.
    ///
    /// The default frames per second is 4, but can be changed on instantiation using the
    /// `with_fps` function.
    pub fn new() -> Self {
        let default = SpriteAnimData::default();
        let mut collection = HashMap::new();
        collection.insert("Default".to_string(), default.clone());
        let fps = 4.0;
        Self {
            collection,
            current: default,
            fps,
            timer: Timer::from_seconds(1.0 / fps, TimerMode::Repeating),
        }
    }

    /// Sets the animation that will play by default if trying to play an animation that does not
    /// exist in the collection.
    pub fn with_default_anim(mut self, anim_data: SpriteAnimData) -> Self {
        self.collection
            .insert("Default".to_string(), anim_data);
        self
    }

    /// Adds an animation to the collection on instantiation
    pub fn with_anim(mut self, anim_data: SpriteAnimData) -> Self {
        self.collection
            .insert(anim_data.name.clone(), anim_data);
        self
    }

    /// Sets framerate on instantiation
    pub fn with_fps(mut self, fps: f32) -> Self {
        self.fps = fps;
        self
    }

    /// Sets the framerate of the controller
    pub fn set_fps(&mut self, fps: f32) {
        self.fps = fps;
        self.timer
            .set_duration(Duration::from_secs_f32(1.0 / self.fps));
    }

    /// Sets the current animation to be played.
    /// The animation must be part of the collection owned by the `SpriteAnimController`.
    pub fn set_current_animation(&mut self, anim_name: &str) -> Result<(), SpriteAnimError> {
        if !self.collection.contains_key(anim_name) {
            self.current = self
                .collection
                .get("Default")
                .expect("Default should exist")
                .clone();
            return Err(SpriteAnimError::DoesNotExist(anim_name.into()));
        }
        self.current = self
            .collection
            .get(anim_name)
            .expect("Animation should exist")
            .clone();
        Ok(())
    }
}

/// System that plays the animation inside the `current` field of the `SpriteAnimController`
fn play_animation(
    mut query: Query<(&mut SpriteAnimController, &mut TextureAtlas)>,
    time: Res<Time>,
) {
    for (mut ac, mut ta) in query.iter_mut() {
        ac.timer.tick(time.delta());
        if ac.timer.just_finished() {
            match ac.current.get_frame() {
                Ok(frame) => {
                    ta.index = *frame;
                }
                Err(err) => {
                    error!("{}", err)
                }
            }
        }
    }
}

/// Bundle of components required to display an animated sprite using the `SpriteAnimController`
///
/// Example usage:
/// ```Rust
/// // Create the sprite bundle
/// let texture = asset_server.load("adventurer.png");
/// let sprite_bundle = SpriteBundle {
///     texture,
///     ..Default::default()
/// };
///
/// // Create the texture atlas and layout
/// let layout = TextureAtlasLayout::from_grid(UVec2::new(50, 37), 7, 11, None, None);
/// let layout_handle = texture_atlas_layouts.add(layout);
/// let texture_atlas = TextureAtlas {
///     layout: layout_handle,
///     index: 0,
/// };
///
/// // Create the animations
/// let idle_anim = SpriteAnimData::new("Idle".to_string(), vec![1, 2, 3]);
/// let run_anim = SpriteAnimData::new("Run".to_string(), vec![8, 9, 10, 11, 12, 13]);
///
/// // Create the controller and add the animations
/// let mut controller = SpriteAnimController::new()
///     .with_anim(idle_anim.clone())
///     .with_anim(run_anim)
///     .with_default_anim(idle_anim);
/// controller.set_current_animation("Idle");
///
/// // Spawn the `AnimatedSpriteBundle`
/// commands.spawn(AnimatedSpriteBundle {
///     sprite_bundle,
///     controller,
///     texture_atlas,
/// });
/// ```
#[derive(Bundle)]
pub struct AnimatedSpriteBundle {
    pub sprite_bundle: SpriteBundle,
    pub controller: SpriteAnimController,
    pub texture_atlas: TextureAtlas,
}

/// Plugin that adds sprite animation controls to a sprite.
/// To use, the easiest way is to spawn an `AnimatedSpriteBundle`.
pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, play_animation);
    }
}
