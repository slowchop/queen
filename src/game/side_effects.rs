//! Side effects!
//!
//! Example side effects might be (in english):
//!
//!   "All new ants will have a 50% of being a random ant type"
//!   "All new ants will walk 5x faster"
//!   "All new ants will get hungry 3x faster"
//!   "All new ants will get hungry 2x slower"
//!   "All new ants will eat 3x faster"
//!   "All new ants will eat 3x as much food"
//!   "All new ants will eat 2x slower"
//!   "All ants will get squished outside 2x as often"
//!   "All ants will get squished outside 2x as often"
//!   "The Queen will eat passing by ants when starving 2x as often"
//!   "The Queen will increase egg laying speed by 2x"
//!   "The Queen will decrease egg laying speed by 3x"
//!   "The Queen's eggs will take 5x longer to hatch"
//!   "The Queen's eggs will be 2x as less likely to hatch"
//!   "Scout ants will take 2x longer to find new food"
//!   "Scout ants will take 3x less time to find new food"
//!   "Cargo ants will take 2x longer to gather food"
//!   "Cargo ants will lose half the food they gather"
//!
//! Higher score the better for the player.
//!
use std::hash::{Hash, Hasher};
use std::time::Duration;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use strum::{EnumCount, EnumDiscriminants};
use crate::game::food::FoodInfo;
use crate::game::time::GameTime;

/// A side effect applied to an entity. One per food. The component is [AppliedFoodSideEffects].
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct AppliedFoodSideEffect {
    pub food: FoodInfo,
    pub timeout_at: Duration,
}

/// All the side effects applied.
#[derive(Component, Deref, DerefMut)]
pub struct AppliedFoodSideEffects(Vec<AppliedFoodSideEffect>);

impl AppliedFoodSideEffects {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add_or_update(&mut self, food: FoodInfo, timeout_at: Duration) {
        if let Some(existing) = self.0.iter_mut().find(|existing| existing.food == food) {
            existing.timeout_at = timeout_at;
        } else {
            self.0.push(AppliedFoodSideEffect { food, timeout_at });
        }
    }

    pub fn remove_expired(&mut self, time: &Duration) {
        self.0.retain(|existing| existing.timeout_at > *time);
    }

    /// Work out the total multipliers for each side effect.
    pub fn calculate_totals(&self) -> CalculatedSideEffects {
        let mut calculated_side_effects = CalculatedSideEffects::new();
        for applied_side_effect in self.0.iter() {
            for side_effect in &applied_side_effect.food.side_effects {
                calculated_side_effects.apply(side_effect);
            }
        };
        calculated_side_effects
    }
}

/// The total side effects combined for this entity.
#[derive(Component)]
pub struct CalculatedSideEffects(HashMap<SideEffectDiscriminants, SideEffect>);

impl CalculatedSideEffects {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn apply(&mut self, side_effect: &SideEffect) {
        let discriminant: SideEffectDiscriminants = side_effect.into();
        match self.0.get_mut(&discriminant) {
            Some(existing) => {
                existing.apply_or_panic(side_effect);
            }
            None => {
                self.0.insert(discriminant, side_effect.clone());
            }
        }
    }

    pub fn as_float(&self, side_effect: SideEffectDiscriminants) -> f32 {
        self.0.get(&side_effect).map(|effect| effect.as_float()).unwrap_or(1f32)
    }
}

pub fn calculate_total_side_effects(mut query: Query<(&AppliedFoodSideEffects, &mut CalculatedSideEffects)>) {
    for (applied, mut calculated) in query.iter_mut() {
        *calculated = applied.calculate_totals();
    }
}

pub fn remove_expired_side_effects(
    time: Res<GameTime>,
    mut applied_side_effects: Query<&mut AppliedFoodSideEffects>,
) {
    let time = time.since_startup();
    for mut applied in applied_side_effects.iter_mut() {
        applied.remove_expired(&time);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, EnumCount, EnumDiscriminants)]
#[strum_discriminants(derive(Hash))]
pub enum SideEffect {
    QueenEggRate(Multiplier),
    QueenHungerRate(Multiplier),
    AntHungerRate(Multiplier),
    AntSpeed(Multiplier),
    AntSquishRate(Multiplier),
}

impl SideEffect {
    /// Would be cool if certain foods or flavours gave specific effects.
    pub fn random() -> Self {
        let random = rand::random::<u32>() % SideEffect::COUNT as u32;
        match random {
            0 => Self::QueenEggRate(Multiplier::random()),
            1 => Self::QueenHungerRate(Multiplier::random()),
            2 => Self::AntHungerRate(Multiplier::random()),
            3 => Self::AntSpeed(Multiplier::random()),
            4 => Self::AntSquishRate(Multiplier::random()),
            _ => unreachable!(),
        }
    }

    pub fn score(&self) -> f32 {
        match self {
            Self::QueenEggRate(multiplier) => 2f32 * multiplier.score(),
            Self::QueenHungerRate(multiplier) => -3f32 * multiplier.score(),
            Self::AntHungerRate(multiplier) => -3f32 * multiplier.score(),
            Self::AntSpeed(multiplier) => 3f32 * multiplier.score(),
            Self::AntSquishRate(multiplier) => -2f32 * multiplier.score(),
        }
    }

    pub fn as_float(&self) -> f32 {
        let maybe_multiplier = self.get_multiplier();
        let multiplier = maybe_multiplier.as_ref().unwrap();
        multiplier.as_float()
    }

    /// Will multiply with another side effect of the same type.
    pub fn apply_or_panic(&mut self, other: &SideEffect) {
        let this_discriminant: SideEffectDiscriminants = (*self).into();
        let other_discriminant: SideEffectDiscriminants = other.into();
        debug_assert_eq!(this_discriminant, other_discriminant);

        let Some(this_multiplier) = self.get_mut_multiplier() else {
            todo!("Something isn't am multiplier");
        };

        let Some(other_multiplier) = other.get_multiplier() else {
            todo!("Other isn't am multiplier");
        };

        this_multiplier.apply(other_multiplier);
    }

    pub fn get_multiplier(&self) -> Option<&Multiplier> {
        match self {
            Self::QueenEggRate(multiplier) => Some(multiplier),
            Self::QueenHungerRate(multiplier) => Some(multiplier),
            Self::AntHungerRate(multiplier) => Some(multiplier),
            Self::AntSpeed(multiplier) => Some(multiplier),
            Self::AntSquishRate(multiplier) => Some(multiplier),
        }
    }

    pub fn get_mut_multiplier(&mut self) -> Option<&mut Multiplier> {
        match self {
            Self::QueenEggRate(multiplier) => Some(multiplier),
            Self::QueenHungerRate(multiplier) => Some(multiplier),
            Self::AntHungerRate(multiplier) => Some(multiplier),
            Self::AntSpeed(multiplier) => Some(multiplier),
            Self::AntSquishRate(multiplier) => Some(multiplier),
        }
    }

    pub fn short_name(&self) -> String {
        let mut s = String::new();

        match self {
            SideEffect::AntHungerRate(multiplier) => {
                s.push_str("Ant Hunger ");
                multiplier.short_name_mutate(&mut s);
            }
            SideEffect::AntSpeed(multiplier) => {
                s.push_str("Ant Movement ");
                multiplier.short_name_mutate(&mut s);
            }
            SideEffect::AntSquishRate(multiplier) => {
                s.push_str("Ant Squish ");
                multiplier.short_name_mutate(&mut s);
            }
            SideEffect::QueenEggRate(multiplier) => {
                s.push_str("Queen Egg Production ");
                multiplier.short_name_mutate(&mut s);
            }
            SideEffect::QueenHungerRate(multiplier) => {
                s.push_str("Queen Hunger ");
                multiplier.short_name_mutate(&mut s);
            }
        }

        s
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Multiplier {
    IncreaseBy(f32),
    DecreaseBy(f32),
}

impl Hash for Multiplier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::IncreaseBy(value) => {
                0u8.hash(state);
                value.to_bits().hash(state);
            }
            Self::DecreaseBy(value) => {
                1u8.hash(state);
                value.to_bits().hash(state);
            }
        }
    }
}

impl Multiplier {
    pub fn random() -> Self {
        let random = rand::random::<u32>() % 2;
        let possible = [2f32, 3f32];
        let random_index = rand::random::<usize>() % possible.len();
        let amount = possible[random_index];

        match random {
            0 => Self::IncreaseBy(amount),
            1 => Self::DecreaseBy(amount),
            _ => unreachable!(),
        }
    }

    pub fn from_float(value: f32) -> Self {
        if value > 1f32 {
            Self::IncreaseBy(value)
        } else {
            Self::DecreaseBy(1f32 / value)
        }
    }

    pub fn as_float(&self) -> f32 {
        match self {
            Self::IncreaseBy(amount) => *amount as f32,
            Self::DecreaseBy(amount) => 1f32 / *amount as f32,
        }
    }

    pub fn apply(&mut self, other: &Self) {
        let this_float = self.as_float();
        let other_float = other.as_float();
        let new_float = this_float * other_float;
        *self = Self::from_float(new_float);
    }

    pub fn score(&self) -> f32 {
        self.as_float()
    }

    pub fn short_name(&self) -> String {
        let mut s = String::new();
        self.short_name_mutate(&mut s);
        s
    }

    pub fn short_name_mutate(&self, mut s: &mut String) {
        match self {
            Multiplier::IncreaseBy(n) => {
                s.push_str("x");
                s.push_str(n.to_string().as_str());
            }
            Multiplier::DecreaseBy(n) => {
                s.push_str("/");
                s.push_str(n.to_string().as_str());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::food_types::FoodId;
    use super::*;

    #[test]
    fn applied_to_total() {
        let mut applied = AppliedFoodSideEffects(vec![
            AppliedFoodSideEffect {
                food: FoodInfo {
                    food_id: FoodId::random(),
                    side_effects: vec![
                        SideEffect::AntHungerRate(Multiplier::IncreaseBy(2f32)),
                        SideEffect::AntHungerRate(Multiplier::IncreaseBy(3f32)),
                    ],
                },
                timeout_at: Default::default(),
            }]);

        let total = applied.calculate_totals();
        assert_eq!(total.as_float(SideEffectDiscriminants::AntHungerRate), 6f32);
        assert_eq!(total.as_float(SideEffectDiscriminants::QueenEggRate), 1f32);
    }
}

#[test]
fn short_names() {
    let fixtures = vec![
        (
            "Ant Hunger x2",
            SideEffect::AntHungerRate(Multiplier::IncreaseBy(2f32))
        ),
        (
            "Ant Hunger /2",
            SideEffect::AntHungerRate(Multiplier::DecreaseBy(2f32))
        ),
        (
            "Queen Hunger x2",
            SideEffect::QueenHungerRate(Multiplier::IncreaseBy(2f32))
        ),
    ];

    for f in fixtures {
        assert_eq!(f.0, f.1.short_name());
    }
}
