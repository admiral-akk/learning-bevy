use bevy::prelude::{Commands, Component, Entity, Query, Resource, With};

pub fn clean<C: Component>(mut commands: Commands, entities: Query<Entity, With<C>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn remove<R: Resource>(mut commands: Commands) {
    commands.remove_resource::<R>()
}
