use bevy::prelude::*;

use crate::components::{
    damage::Damage,
    heal::Heal,
    health::{Alive, Dead, Health},
};

///Plugin for calculating and applying damage
pub struct HealthAndDamagePlugin;

impl Plugin for HealthAndDamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (apply_heal, apply_damage).chain());
    }
}

///apply_damage queries for all entities that need a Damage calculation then performs and applies that calculation
fn apply_damage(
    mut command: Commands,
    mut query: Query<(Entity, &mut Health, &Damage), With<Alive>>,
) {
    for (entity, mut hp, damage) in query.iter_mut() {
        if damage.value >= hp.current {
            command.entity(entity).remove::<Alive>();
            command.entity(entity).insert(Dead);
            hp.current = 0;
        } else {
            hp.current -= damage.value;
        }

        command.entity(entity).remove::<Damage>();
    }
}

///apply_damage queries for all entities that need a Heal calculation then performs and applies that calculation
fn apply_heal(mut command: Commands, mut query: Query<(Entity, &mut Health, &Heal), With<Alive>>) {
    for (entity, mut hp, heal) in query.iter_mut() {
        if heal.value >= (hp.maximum - hp.current) {
            hp.current = hp.maximum;
        } else {
            hp.current += heal.value;
        }

        command.entity(entity).remove::<Heal>();
    }
}
