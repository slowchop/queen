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

pub enum SideEffect {
    NewAnts(EffectType),
    AllAnts(EffectType),
    Queen(EffectType),
}

impl SideEffect {
    pub fn short_name(&self) -> String {
        let mut s = String::new();

        let effect_type = match self {
            SideEffect::NewAnts(effect_type) => {
                s.push_str("New: ");
                effect_type
            }
            SideEffect::AllAnts(effect_type) => {
                s.push_str("All: ");
                effect_type
            }
            SideEffect::Queen(effect_type) => {
                s.push_str("Queen: ");
                effect_type
            }
        };

        effect_type.short_name_mutate(&mut s);

        s
    }
}

pub enum EffectType {
    WalkSpeed(Multiplier),
    HatchRate(Multiplier),
    SquishRate(Multiplier),
}

impl EffectType {
    pub fn short_name(&self) -> String {
        let mut s = String::new();
        self.short_name_mutate(&mut s);
        s
    }

    pub fn short_name_mutate(&self, mut s: &mut String) {
        let multiplier = match self {
            EffectType::WalkSpeed(multiplier) => {
                s.push_str("Walk ");
                multiplier
            }
            EffectType::HatchRate(multiplier) => {
                s.push_str("Hatch ");
                multiplier
            }
            EffectType::SquishRate(multiplier) => {
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
