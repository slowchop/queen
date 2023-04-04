use crate::game::positions::SideIPos;

enum FoodType {
    Water,
    Apple,
    Banana,
    Carrot,
    Fly,
    Worm,
    Coffee,
    Manure,
    MedicinePill,
    Honey,
    Sugar,
}

pub struct FoundFood {
    food: FoodType,
    position: SideIPos,
    time: f32,
}

pub struct ApprovedFood(Vec<FoundFood>);
