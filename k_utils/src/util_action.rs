use bevy::prelude::{App, Commands, Component, Entity, Plugin, Query};
use iyes_loopless::{condition::ConditionalSystemDescriptor, prelude::ConditionSet};

use super::{
    util_stages::{APPLY_MOVE, SIMULATE_MOVES},
    util_state::{StateContraint, UtilState},
    util_systems::clean,
};

#[derive(Component)]
pub struct Proposal {
    _private: (),
    pub should_act: bool,
    pub priority: usize,
}

#[derive(Component)]
pub struct Selection {
    _private: (),
}
impl Proposal {
    pub fn propose_action<C: Component + Send + Sync + 'static>(
        commands: &mut Commands,
        should_act: bool,
        priority: usize,
        action: C,
    ) {
        commands
            .spawn(Proposal {
                _private: (),
                should_act,
                priority,
            })
            .insert(action);
    }
}

pub fn select_move(mut commands: Commands, proposals: Query<(Entity, &Proposal)>) {
    let mut proposals = proposals.iter().collect::<Vec<_>>();
    proposals.sort_by_key(|(_, proposal)| proposal.priority);

    for (index, &(entity, proposal)) in proposals.iter().enumerate() {
        if index == 0 {
            if proposal.should_act {
                commands.entity(entity).insert(Selection { _private: () });
            }
        } else {
            commands.entity(entity).despawn();
        }
    }
}

impl Selection {
    pub fn select_action<C: Component + Send + Sync + 'static>(commands: &mut Commands, action: C) {
        commands.spawn(Selection { _private: () }).insert(action);
    }
}

pub fn handle_actions<StateType: StateContraint>(
    app: &mut App,
    simulate: ConditionalSystemDescriptor,
    apply: ConditionalSystemDescriptor,
) {
    app.add_system_set_to_stage(
        SIMULATE_MOVES,
        ConditionSet::new()
            .run_in_state(UtilState::<StateType>::Running)
            .with_system(simulate)
            .into(),
    );
    app.add_system_set_to_stage(
        APPLY_MOVE,
        ConditionSet::new()
            .run_in_state(UtilState::<StateType>::Running)
            .with_system(apply)
            .into(),
    );
}

pub struct ActionPlugin {
    pub clean_up_stage: &'static str,
}

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_to_stage(self.clean_up_stage, clean::<Proposal>)
            .add_system_to_stage(self.clean_up_stage, clean::<Selection>);
    }
}
