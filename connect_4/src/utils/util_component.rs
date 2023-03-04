use std::{fmt::Debug, hash::Hash, ops::Deref};

use bevy::prelude::Component;

#[derive(Component, PartialEq, Clone, Copy, Hash, Eq, Debug)]
pub struct ComponentWrapper<C> {
    c: C,
}

impl<C: Component> Deref for ComponentWrapper<C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.c
    }
}
