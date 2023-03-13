use bevy::prelude::{App, Entity, EventReader, EventWriter, Query, World};
use iyes_loopless::{
    condition::ConditionalSystemDescriptor,
    prelude::{ConditionSet, IntoConditionalSystem},
};

use super::{
    util_stages::APPLY_MOVE,
    util_state::{StateContraint, UtilState},
};

pub trait Action
where
    Self: Sized + Sync + Send + 'static,
{
    fn apply_move() -> ConditionalSystemDescriptor {
        (|| {}).into_conditional()
    }
}

pub fn handle_actions<StateType: StateContraint>(
    app: &mut App,
    apply: ConditionalSystemDescriptor,
) {
    app.add_system_set_to_stage(
        APPLY_MOVE,
        ConditionSet::new()
            .run_in_state(UtilState::<StateType>::Running)
            .with_system(apply)
            .into(),
    );
}
