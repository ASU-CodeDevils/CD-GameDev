use bevy::prelude::*;

///Pointer component to mark entities that need a Damage calculation
#[derive(Component, Debug)]
pub struct Damage {
    pub value: u32,
}

impl Damage {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

///Component to represent how much damage an entity does
#[derive(Component, Debug)]
pub struct DamageFactor {
    pub value: u32,
}

impl DamageFactor {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

///Bundle for all Damage related components
#[derive(Bundle, Debug)]
pub struct DamageBundle {
    pub damage_factor: DamageFactor,
}
