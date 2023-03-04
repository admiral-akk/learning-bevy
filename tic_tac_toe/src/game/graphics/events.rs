use bevy::prelude::Entity;

use crate::game::model::types::{Player};

pub struct Spawn {
    pub parent: Entity,
    pub owner: Player,
}

impl Spawn {
    pub fn new(parent: Entity, owner: Player) -> Spawn {
        Spawn { parent, owner }
    }
}

pub struct Despawn {
    pub parent: Entity,
}

impl Despawn {
    pub fn new(parent: Entity) -> Despawn {
        Despawn { parent }
    }
}

pub struct Animate(pub Entity);
