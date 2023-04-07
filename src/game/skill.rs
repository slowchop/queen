use std::time::Duration;
use crate::game::food::FoodInfo;
use crate::game::food_types::FoodId;
use crate::game::side_effects::{SideEffect, SideEffectDiscriminants};

pub enum SkillMode {
    Career,
    Random,
    Jam,
}

impl SkillMode {
    pub fn try_random_unique_side_effects(count: usize) -> Option<Vec<SideEffect>> {
        let mut effects = Vec::new();
        for _ in 0..count {
            let effect = SideEffect::random();

            // First one must be negative.
            if effects.is_empty() && effect.score() > 0f32 {
                return None;
            }

            // Make sure the effect is unique.
            let discriminant: SideEffectDiscriminants = effect.into();
            if effects.iter().any(|e| Into::<SideEffectDiscriminants>::into(e) == discriminant) {
                return None;
            }

            effects.push(effect);
        }
        Some(effects)
    }

    pub fn generate_side_effects_for_score(count: usize, score: f32) -> Vec<SideEffect> {
        let mut range = 0f32;
        loop {
            // Try a few times to match score within range.
            for _ in 0..2 {
                let Some(effects) = Self::try_random_unique_side_effects(count) else {
                    continue;
                };
                let total_score = effects.iter().map(|effect| effect.score()).sum::<f32>();
                let range = (score - range)..=(score + range);
                if range.contains(&total_score) {
                    return effects;
                }
            }

            // If a score can't be found within the range, expand the range slightly.
            range += 1f32;
        }
    }

    pub fn next_food(&self, time_played: Duration) -> FoodInfo {
        let minutes_played = time_played.as_secs_f32() / 60f32;

        let expected_score = match &self {
            SkillMode::Career => {
                // Start with a +50 score and then minus 5 for each food eaten.
                // Every minute the expected score is reduced by 5.
                // Add a variation from -20 to +20
                let variation = (rand::random::<f32>() * 40f32 - 20f32);

                (50f32 - minutes_played * 5f32) + variation
            }
            SkillMode::Jam => {
                // Always hard
                -50f32
            }
            SkillMode::Random => {
                // A random amount of score between -50 and +50.
                rand::random::<f32>() * 100f32 - 50f32
            }
        };

        let food_id = FoodId::random();
        let mut side_effects = Self::generate_side_effects_for_score(2, expected_score);

        // Sort by score.
        side_effects.sort_by(|a, b| a.score().partial_cmp(&b.score()).unwrap());

        FoodInfo {
            food_id,
            side_effects,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_really_a_test() {
        for minutes in 0..50 {
            let food_info = SkillMode::Career.next_food(Duration::from_secs(minutes * 60));
            let total_score = food_info.side_effects.iter().map(|effect| effect.score()).sum::<i32>();
            println!("Minutes: {minutes:?} Score: {total_score:?} Food: {food_info:?}");
        }
    }
}