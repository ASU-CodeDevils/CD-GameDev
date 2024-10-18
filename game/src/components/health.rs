use bevy::prelude::*;

///pointer struct to mark entity as alive
#[derive(Component, Debug)]
pub struct Alive;

///pointer struct to mark entity as dead
#[derive(Component, Debug)]
pub struct Dead;

///Health component to track entity health
#[derive(Component, Debug)]
pub struct Health {
    pub maximum: u32,
    pub current: u32,
}

impl Health {
    pub fn new(maximum: u32) -> Self {
        Self {
            maximum,
            current: maximum,
        }
    }

    pub fn with_current_health(mut self, health: u32) -> Self {
        if health >= self.maximum {
            self.current = self.maximum;
            self
        } else {
            self.current = health;
            self
        }
    }
}

///HealthBundle to bundle all Health related components
#[derive(Bundle, Debug)]
pub struct HealthBundle {
    pub health: Health,
}
