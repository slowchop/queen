use strum::{EnumCount, FromRepr};

///! Side effects!
///!
///! Example side effects might be (in english):
///!   
///!   "All new ants will have a 50% of being a random ant type"
///!   "All new ants will walk 5x faster"
///!   "All new ants will get hungry 3x faster"
///!   "All new ants will get hungry 2x slower"
///!   "All new ants will eat 3x faster"
///!   "All new ants will eat 3x as much food"
///!   "All new ants will eat 2x slower"
///!   "All ants will get squished outside 2x as often"
///!   "All ants will get squished outside 2x as often"
///!   "The Queen will eat passing by ants when starving 2x as often"
///!   "The Queen will increase egg laying speed by 2x"
///!   "The Queen will decrease egg laying speed by 3x"
///!   "The Queen's eggs will take 5x longer to hatch"
///!   "The Queen's eggs will be 2x as less likely to hatch"
///!   "Scout ants will take 2x longer to find new food"
///!   "Scout ants will take 3x less time to find new food"
///!   "Cargo ants will take 2x longer to gather food"
///!   "Cargo ants will lose half the food they gather"

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumCount)]
pub enum SideEffect {
    NewAnts(AntEffectType),
    AllAnts(AntEffectType),
    Queen(QueenEffectType),
}

impl SideEffect {
    /// Would be cool if some foods gave specific effects.
    pub fn random() -> Self {
        let random = rand::random::<u32>() % SideEffect::COUNT;
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

#[derive(EnumCount, FromRepr)]
pub enum QueenEffectType {
    HatchRate(Multiplier),
    HungerRate(Multiplier),
}

impl QueenEffectType {
    pub fn random() -> Self {
        let random = rand::random::<u32>() % QueenEffectType::COUNT;
        match random {
            0 => Self::HatchRate(Multiplier::random()),
            1 => Self::HungerRate(Multiplier::random()),
            _ => unreachable!(),
        }
    }

    pub fn score(&self) -> i32 {

    }
}

#[derive(EnumCount, FromRepr)]
pub enum AntEffectType {
    HungerRate(Multiplier),
    WalkSpeed(Multiplier),
    SquishRate(Multiplier),
}

impl AntEffectType {
    pub fn score(&self) -> i32 {
        match self {
            Self::HungerRate(multiplier) => 20 * multiplier.score(),
            Self::WalkSpeed(multiplier) => 10 * multiplier.score(),
            Self::SquishRate(multiplier) => 5 * multiplier.score(),
        }
    }

    pub fn random() -> Self {
        let random = rand::random::<usize>() % Self::COUNT;
        Self::from_repr(random).unwrap()
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
        };

        multiplier.short_name_mutate(&mut s);
    }
}

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
                SideEffect::NewAnts(EffectType::WalkSpeed(Multiplier::IncreaseBy(2))),
            ),
            (
                "All: Walk /2",
                SideEffect::AllAnts(EffectType::WalkSpeed(Multiplier::DecreaseBy(2))),
            ),
            (
                "Queen: Walk x2",
                SideEffect::Queen(EffectType::WalkSpeed(Multiplier::IncreaseBy(2))),
            ),
            (
                "New: Hatch x2",
                SideEffect::NewAnts(EffectType::HatchRate(Multiplier::IncreaseBy(2))),
            ),
            (
                "All: Hatch /2",
                SideEffect::AllAnts(EffectType::HatchRate(Multiplier::DecreaseBy(2))),
            ),
            (
                "Queen: Hatch x2",
                SideEffect::Queen(EffectType::HatchRate(Multiplier::IncreaseBy(2))),
            ),
            (
                "New: Squish x2",
                SideEffect::NewAnts(EffectType::SquishRate(Multiplier::IncreaseBy(2))),
            ),
            (
                "All: Squish /2",
                SideEffect::AllAnts(EffectType::SquishRate(Multiplier::DecreaseBy(2))),
            ),
            (
                "Queen: Squish x2",
                SideEffect::Queen(EffectType::SquishRate(Multiplier::IncreaseBy(2))),
            ),
        ];

        for f in fixtures {
            assert_eq!(f.0, f.1.short_name());
        }
    }
}
