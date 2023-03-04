use bevy::{
    prelude::{Changed, Entity, Query, RemovedComponents, ResMut, Resource},
    ui::Interaction,
    utils::HashSet,
};

#[derive(Resource, Debug)]
pub struct UiInteraction {
    ui_elements: HashSet<Entity>,
}

impl UiInteraction {
    pub fn new() -> UiInteraction {
        UiInteraction {
            ui_elements: HashSet::new(),
        }
    }

    pub fn captured(&self) -> bool {
        !self.ui_elements.is_empty()
    }
}

pub fn mouse_system(
    mut interaction_query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut mouse: ResMut<UiInteraction>,
) {
    for (entity, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                mouse.ui_elements.insert(entity);
            }
            Interaction::None => {
                mouse.ui_elements.remove(&entity);
            }
            Interaction::Clicked => {}
        }
    }
}

pub fn entity_removed(removed: RemovedComponents<Interaction>, mut mouse: ResMut<UiInteraction>) {
    for entity in removed.iter() {
        mouse.ui_elements.remove(&entity);
    }
}
