use bevy::prelude::App;
use iyes_loopless::{
    condition::{ConditionSystemSet, ConditionalSystemDescriptor},
    prelude::ConditionSet,
};

use super::{
    util_stages::*,
    util_state::{StateContraint, UtilState},
};

pub fn handle_input<StateType: StateContraint>(
    app: &mut App,
    propose: Vec<ConditionalSystemDescriptor>,
) {
    let mut input: ConditionSystemSet = ConditionSet::new()
        .run_in_state(UtilState::<StateType>::Running)
        .into();
    for p in propose.into_iter() {
        input = input.with_system(p);
    }
    app.add_system_set_to_stage(PROPOSE_MOVES, input.into());
}
