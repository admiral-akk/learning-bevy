use bevy::prelude::App;
use iyes_loopless::{
    condition::{ConditionSystemSet, ConditionalSystemDescriptor},
    prelude::ConditionSet,
};

use super::{
    util_stages::UPDATE_GRAPHICS,
    util_state::{StateContraint, UtilState},
};

pub fn update_graphics<StateType: StateContraint>(
    app: &mut App,
    graphics: Vec<ConditionalSystemDescriptor>,
) {
    let mut system_set: ConditionSystemSet = ConditionSet::new()
        .run_in_state(UtilState::<StateType>::Running)
        .into();
    for graphic_system in graphics.into_iter() {
        system_set = system_set.with_system(graphic_system);
    }

    app.add_system_set_to_stage(UPDATE_GRAPHICS, system_set.into());
}
