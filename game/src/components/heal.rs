use bevy::prelude::*;

///Pointer struct to mark components that need a Heal calculation
#[derive(Component, Debug)]
pub struct Heal {
    pub value: u32,
}

impl Heal {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

///Component that represents the healing factor
#[derive(Component, Debug)]
pub struct HealFactor {
    pub value: u32,
}

impl HealFactor {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

///Bundle for all Heal related components
#[derive(Bundle, Debug)]
pub struct HealBundle {
    pub heal_factor: HealFactor,
}
