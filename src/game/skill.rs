use std::time::Duration;
use crate::game::food::FoodInfo;
use crate::game::side_effects::SideEffect;

pub enum SkillMode {
    Career,
    Random,
    Hard,
}

impl SkillMode {
    pub fn random(count: usize) -> Vec<SideEffect> {
        let mut effects = Vec::new();
        for _ in 0..count {
            effects.push(SideEffect::random());
        }
        effects
    }

    pub fn generate_for_score(count: usize, score: i32) -> Vec<SideEffect> {
        let range = 0;
        loop {
            // Try 10 times to match score within range.
            for _ in 0..10 {
                let effects = Self::random(count);
                let total_score = effects.iter().map(|effect| effect.score()).sum::<i32>();
                if total_score >= score - range && total_score <= score + range {
                    return effects;
                }

            }

            // If a score can't be found within the range, expand the range slightly.
            range += 1;
        }
    }

    pub fn next_food(&self, time_played: Duration) -> FoodInfo {
        match &self {
            SkillMode::Career => {}
            SkillMode::Random => {}
            SkillMode::Hard => {}
        }

    }
}