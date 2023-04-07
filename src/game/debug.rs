use std::time::Duration;
use crate::game::food::{DiscoveredFood, FoodState};
use crate::game::positions::SideIPos;
use bevy::prelude::*;
use crate::game::skill::SkillMode;

pub fn check_for_f3_to_offer_queen_new_food(skill_mode: Res<SkillMode>, mut food_state: ResMut<FoodState>, keyboard_input: Res<Input<KeyCode>>) {
    if !keyboard_input.just_pressed(KeyCode::F3) {
        return;
    }

    let food_info = skill_mode.next_food(Duration::ZERO);

    // Give the ant some food to carry.
    let discovered = DiscoveredFood {
        food_info,
        position: SideIPos::new(0, 0),
        time_to_discover: Duration::ZERO,
        stash_remaining: 1000f32,
    };

    warn!(?discovered);

    food_state.approve_food(discovered);
}