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
use strum::{EnumCount, FromRepr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumCount)]
pub enum SideEffect {
    NewAnts(AntEffectType),
    AllAnts(AntEffectType),
    Queen(QueenEffectType),
}

impl SideEffect {
    /// Would be cool if some foods gave specific effects.
    pub fn random() -> Self {
        let random = rand::random::<u32>() % SideEffect::COUNT as u32;
        match random {
            0 => Self::NewAnts(AntEffectType::random()),
            1 => Self::AllAnts(AntEffectType::random()),
            2 => Self::Queen(QueenEffectType::random()),
            _ => unreachable!(),
        }
    }

    pub fn score(&self) -> i32 {
        match self {
            Self::NewAnts(effect) => effect.score(),
            Self::AllAnts(effect) => effect.score(),
            Self::Queen(effect) => effect.score(),
        }
    }

    pub fn unique_id(&self) -> String {
        match self {
            Self::NewAnts(effect) => effect.unique_id_without_multiplier(),
            Self::AllAnts(effect) => effect.unique_id_without_multiplier(),
            Self::Queen(effect) => effect.unique_id_without_multiplier(),
        }
    }

    pub fn short_name(&self) -> String {
        let mut s = String::new();

        match self {
            SideEffect::NewAnts(effect_type) => {
                s.push_str("New: ");
                effect_type.short_name_mutate(&mut s);
            }
            SideEffect::AllAnts(effect_type) => {
                s.push_str("All: ");
                effect_type.short_name_mutate(&mut s);
            }
            SideEffect::Queen(effect_type) => {
                s.push_str("Queen: ");
                effect_type.short_name_mutate(&mut s);
            }
        }

        s
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumCount)]
pub enum QueenEffectType {
    HatchRate(Multiplier),
    HungerRate(Multiplier),
}

impl QueenEffectType {
    pub fn random() -> Self {
        let random = rand::random::<u32>() % QueenEffectType::COUNT as u32;
        match random {
            0 => Self::HatchRate(Multiplier::random()),
            1 => Self::HungerRate(Multiplier::random()),
            _ => unreachable!(),
        }
    }

    pub fn unique_id_without_multiplier(&self) -> String {
        match self {
            Self::HatchRate(_) => format!("QueenHatchRate"),
            Self::HungerRate(_) => format!("QueenHungerRate"),
        }
    }

    pub fn score(&self) -> i32 {
        match self {
            Self::HatchRate(multiplier) => 2 * multiplier.score(),
            Self::HungerRate(multiplier) => -3 * multiplier.score(),
        }
    }

    pub fn short_name_mutate(&self, mut s: &mut String) {
        let multiplier = match self {
            Self::HatchRate(multiplier) => {
                s.push_str("Hatch Rate ");
                multiplier
            }
            Self::HungerRate(multiplier) => {
                s.push_str("Hunger Rate ");
                multiplier
            }
        };

        multiplier.short_name_mutate(&mut s);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumCount)]
pub enum AntEffectType {
    HungerRate(Multiplier),
    WalkSpeed(Multiplier),
    SquishRate(Multiplier),
}

impl AntEffectType {
    pub fn score(&self) -> i32 {
        match self {
            Self::HungerRate(multiplier) => -3 * multiplier.score(),
            Self::WalkSpeed(multiplier) => 3 * multiplier.score(),
            Self::SquishRate(multiplier) => -2 * multiplier.score(),
        }
    }

    pub fn random() -> Self {
        let random = rand::random::<usize>() % Self::COUNT;
        match random {
            0 => Self::HungerRate(Multiplier::random()),
            1 => Self::WalkSpeed(Multiplier::random()),
            2 => Self::SquishRate(Multiplier::random()),
            _ => unreachable!(),
        }
    }

    pub fn unique_id_without_multiplier(&self) -> String {
        match self {
            Self::HungerRate(_) => format!("AntHungerRate"),
            Self::WalkSpeed(_) => format!("AntWalkSpeed"),
            Self::SquishRate(_) => format!("AntSquishRate"),
        }
    }

    pub fn short_name(&self) -> String {
        let mut s = String::new();
        self.short_name_mutate(&mut s);
        s
    }

    pub fn short_name_mutate(&self, mut s: &mut String) {
        let multiplier = match self {
            Self::WalkSpeed(multiplier) => {
                s.push_str("Walk ");
                multiplier
            }
            Self::SquishRate(multiplier) => {
                s.push_str("Squish ");
                multiplier
            }
            AntEffectType::HungerRate(multiplier) => {
                s.push_str("Hunger ");
                multiplier
            }
        };

        multiplier.short_name_mutate(&mut s);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Multiplier {
    IncreaseBy(u8),
    DecreaseBy(u8),
}

impl Multiplier {
    /// Increase or decrease by range 2x, 3x, 4x, 5x
    pub fn random() -> Self {
        let random = rand::random::<u32>() % 2;
        let amount = (rand::random::<u32>() % 4 + 2) as u8;
        match random {
            0 => Self::IncreaseBy(amount),
            1 => Self::DecreaseBy(amount),
            _ => unreachable!(),
        }
    }

    pub fn score(&self) -> i32 {
        match self {
            Self::IncreaseBy(amount) => *amount as i32,
            Self::DecreaseBy(amount) => -(*amount as i32),
        }
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
    use super::*;

    #[test]
    fn short_names() {
        let fixtures = vec![
            (
                "New: Walk x2",
                SideEffect::NewAnts(AntEffectType::WalkSpeed(Multiplier::IncreaseBy(2))),
            ),
            (
                "All: Walk /2",
                SideEffect::AllAnts(AntEffectType::WalkSpeed(Multiplier::DecreaseBy(2))),
            ),
            (
                "Queen: Walk x2",
                SideEffect::Queen(QueenEffectType::HatchRate(Multiplier::IncreaseBy(2))),
            ),
        ];

        for f in fixtures {
            assert_eq!(f.0, f.1.short_name());
        }
    }
}
