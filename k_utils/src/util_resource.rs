use bevy::prelude::{App, Commands, FromWorld, Resource};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

use super::util_state::{StateContraint, UtilState};

pub fn add_util_resource<StateType: StateContraint, ResourceType: Resource + FromWorld>(
    app: &mut App,
) {
    app.add_enter_system_set(
        UtilState::<StateType>::Running,
        ConditionSet::new()
            .with_system(|mut commands: Commands| commands.init_resource::<ResourceType>())
            .into(),
    );
    app.add_enter_system_set(
        UtilState::<StateType>::Uninitialized,
        ConditionSet::new()
            .with_system(|mut commands: Commands| commands.remove_resource::<ResourceType>())
            .into(),
    );
}
