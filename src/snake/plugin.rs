use bevy::prelude::*;
use iyes_loopless::prelude::*;

use std::collections::VecDeque;
use crate::core::{GameState, GridPosition};

use super::head;
use super::tail;
use super::controller;
use super::helpers;

pub struct SnakePlugin {
    pub init_params: helpers::InitParams
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
struct FixedUpdate;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {

        let fixed_time_step = SystemStage::parallel()
            .with_system_set(ConditionSet::new()
                .run_in_state(GameState::RUNNING)
                .label("move")
                .with_system(controller::move_head)
                .into())
            .with_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::RUNNING)
                    .after("move")

                    .with_system(controller::check_collide_with_food)
                    .with_system(controller::check_for_bite_self)
                    .into());

        app
            .insert_resource(VecDeque::<GridPosition>::new())
            .insert_resource(self.init_params.clone())
            .add_enter_system(GameState::RUNNING, helpers::add_snake)
            .add_exit_system(GameState::DEAD, helpers::cleanup_snake)
            .add_enter_system(GameState::DEAD, helpers::set_death_sprites)
            .add_stage_before(
                CoreStage::Update,
                FixedUpdate,
                FixedTimestepStage::new( self.init_params.movement_time_step)
                    .with_stage(fixed_time_step)
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::RUNNING)
                    .with_system(controller::handle_input)
                    .with_system(controller::consume_food)
                    .with_system(head::snake_head_sprite_position)
                    .with_system(tail::snake_tail_sprite_positions)
                    .into()
            );
    }
}
