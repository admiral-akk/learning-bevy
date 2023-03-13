use crate::util_action::Action;
use crate::util_button::{emit_proposal, update_buttons};

use super::raycast::plugin::RaycastPlugin;
use super::util_stages::*;
use super::util_state::{StateContraint, UtilState};
use super::util_systems::clean;
use bevy::prelude::{App, Commands, Component, Entity, Plugin, Query, Without};
use iyes_loopless::prelude::ConditionSet;
use iyes_loopless::{prelude::AppLooplessStateExt, state::NextState};

#[derive(Component)]
struct Owned;

use bevy::prelude::{CoreStage, SystemStage};

// Run any pre-input logic
const PRE_INPUT: &str = "pre_input";
// Select a move
const SELECT_MOVE: &str = "select_move";
// Mark any newly added entities as owned
const MARK_OWNED: &str = "mark_owned";
// Update state
const UPDATE_STATE: &str = "update_state";
// Remove entites
const CLEAN_UP: &str = "cleanup";
const STAGE_ORDER: &[&str] = &[
    PRE_INPUT,
    PROPOSE_MOVES,
    SIMULATE_MOVES,
    SELECT_MOVE,
    APPLY_MOVE,
    UPDATE_GRAPHICS,
    MARK_OWNED,
    UPDATE_STATE,
    CLEAN_UP,
    UPDATE_DELETED,
];

pub fn add_stages(app: &mut App) {
    for (index, &stage) in STAGE_ORDER.iter().enumerate() {
        if index == 0 {
            app.add_stage_after(CoreStage::PostUpdate, stage, SystemStage::parallel());
        } else {
            app.add_stage_after(STAGE_ORDER[index - 1], stage, SystemStage::parallel());
        }
    }
}
pub struct UtilPluginStruct;

impl Plugin for UtilPluginStruct {
    fn build(&self, app: &mut App) {
        app.add_plugin(RaycastPlugin {
            pre_input_stage: PRE_INPUT,
            update_changed_stage: UPDATE_DELETED,
        })
        .add_system_set_to_stage(
            PRE_INPUT,
            ConditionSet::new()
                .label("UPDATE_BUTTONS")
                .with_system(update_buttons)
                .into(),
        );
    }
}

pub trait UtilPlugin<StateType: StateContraint + Component, ActionType: Action + Clone> {
    fn add_defaults(app: &mut App) {
        app.add_loopless_state_after_stage(UPDATE_STATE, UtilState::<StateType>::Uninitialized);
        app.add_event::<ActionType>();
        app.add_system_set_to_stage(
            PRE_INPUT,
            ConditionSet::new()
                .after("UPDATE_BUTTONS")
                .with_system(emit_proposal::<ActionType>)
                .into(),
        );
        app.add_system_set_to_stage(
            CLEAN_UP,
            ConditionSet::new()
                .run_in_state(UtilState::<StateType>::Uninitialized)
                .with_system(clean::<StateType>)
                .into(),
        );
        app.add_system_set_to_stage(
            UPDATE_STATE,
            ConditionSet::new()
                .run_in_state(UtilState::<StateType>::Enter)
                .with_system(|mut commands: Commands| {
                    commands.insert_resource(NextState(UtilState::<StateType>::Running))
                })
                .into(),
        );

        app.add_system_set_to_stage(
            SELECT_MOVE,
            ConditionSet::new()
                .run_in_state(UtilState::<StateType>::Running)
                .with_system(ActionType::apply_move())
                .into(),
        );
        app.add_system_set_to_stage(
            MARK_OWNED,
            ConditionSet::new()
                .run_in_state(UtilState::<StateType>::Running)
                .with_system(mark_owned::<StateType>)
                .into(),
        );
        app.add_system_set_to_stage(
            CLEAN_UP,
            ConditionSet::new()
                .run_in_state(UtilState::<StateType>::Uninitialized)
                .with_system(clean::<StateType>)
                .into(),
        );
    }

    fn enter_system(mut commands: Commands) {
        Self::enter(&mut commands);
    }

    fn paused_system(mut commands: Commands) {
        Self::paused(&mut commands);
    }
    fn exit_system(mut commands: Commands) {
        Self::exit(&mut commands);
    }

    fn enter(commands: &mut Commands) {
        commands.insert_resource(NextState(UtilState::<StateType>::Enter));
    }
    fn paused(commands: &mut Commands) {
        commands.insert_resource(NextState(UtilState::<StateType>::Paused));
    }
    fn exit(commands: &mut Commands) {
        commands.insert_resource(NextState(UtilState::<StateType>::Uninitialized));
    }

    fn exit_to<OtherStateType: StateContraint + Component>(commands: &mut Commands) {
        commands.insert_resource(NextState(UtilState::<StateType>::Uninitialized));
        commands.insert_resource(NextState(UtilState::<OtherStateType>::Enter));
    }

    fn pause_to<OtherStateType: StateContraint + Component>(commands: &mut Commands) {
        commands.insert_resource(NextState(UtilState::<StateType>::Paused));
        commands.insert_resource(NextState(UtilState::<OtherStateType>::Enter));
    }
}

fn mark_owned<C: Default + Component>(
    mut commands: Commands,
    spawned: Query<Entity, Without<Owned>>,
) {
    for entity in spawned.iter() {
        commands.entity(entity).insert(Owned).insert(C::default());
    }
}
