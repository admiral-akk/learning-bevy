use bevy::prelude::{App, EventReader, EventWriter, Plugin};
use iyes_loopless::{condition::ConditionalSystemDescriptor, prelude::ConditionSet};

use super::{
    util_stages::{APPLY_MOVE, SIMULATE_MOVES},
    util_state::{StateContraint, UtilState},
};

pub struct Proposal<ActionType: 'static + Sync + Send> {
    pub priority: i32,
    pub action: ActionType,
}

pub struct Act<ActionType: 'static + Sync + Send> {
    pub action: ActionType,
}

pub fn select_move<ActionType: 'static + Sync + Send + Clone>(
    mut proposals: EventReader<Proposal<ActionType>>,
    mut action: EventWriter<Act<ActionType>>,
) {
    let mut best = None;
    for proposal in proposals.iter() {
        if best.is_none() {
            best = Some(proposal);
        } else if best.unwrap().priority > proposal.priority {
            best = Some(proposal);
        }
    }
    if let Some(proposal) = best {
        action.send(Act {
            action: proposal.action.clone(),
        })
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
