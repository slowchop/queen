use bevy::prelude::Deref;
use rand::prelude::IteratorRandom;
use std::fmt::{Display, Formatter};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FoodId {
    flavor: FoodFlavor,
    texture: FoodTexture,
    food_type: FoodType,
}

impl FoodId {
    pub fn random() -> Self {
        Self {
            flavor: FoodFlavor::iter().choose(&mut rand::thread_rng()).unwrap(),
            texture: FoodTexture::iter().choose(&mut rand::thread_rng()).unwrap(),
            food_type: FoodType::iter().choose(&mut rand::thread_rng()).unwrap(),
        }
    }
}

/// Display should be "[flavor] [texture] [food_type]", e.g. "Tasteless Soggy Apple"
impl Display for FoodId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?} {:?}",
            self.flavor, self.texture, self.food_type
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter)]
pub enum FoodFlavor {
    Bitter,
    Salty,
    Sour,
    Spicy,
    Sweet,
    Tasteless,
    Tasty,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter)]
pub enum FoodTexture {
    Burnt,
    Chewy,
    Crunchy,
    Dry,
    Frozen,
    Juicy,
    Melted,
    Raw,
    Ripe,
    Rotten,
    Smooth,
    Soggy,
    Sticky,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter)]
pub enum FoodType {
    Almond,
    Anchovy,
    Apple,
    Avocado,
    Bacon,
    Banana,
    Bean,
    Bocadillo,
    Bread,
    Broccoli,
    Burger,
    Cabbage,
    Cake,
    Cantaloupe,
    Carrot,
    Cashew,
    Cauliflower,
    Caviar,
    Cheese,
    Chicken,
    Chips,
    Chocolate,
    Clam,
    Cocada,
    Coconut,
    Cod,
    Coffee,
    Corn,
    Crab,
    Cucumber,
    Curry,
    Egg,
    Eggplant,
    Emu,
    Fish,
    Fly,
    Frog,
    Garlic,
    Grape,
    Grapefruit,
    Ham,
    Hazelnut,
    Herring,
    Honey,
    Honeydew,
    Ice,
    IceCream,
    Kangaroo,
    Ketchup,
    Kiwi,
    Lemon,
    Lettuce,
    Lobster,
    Lychee,
    Mackerel,
    Mango,
    Manure,
    Mayonnaise,
    Meat,
    MedicinePill,
    Milk,
    Mushroom,
    Mustard,
    Noodles,
    Olive,
    Onion,
    Orange,
    Oyster,
    Papaya,
    Pasta,
    Peach,
    Peanut,
    Pear,
    Pepper,
    Pepperoni,
    Pickle,
    Pineapple,
    Pistachio,
    Pizza,
    Plum,
    Potato,
    Prosciutto,
    Rice,
    Salad,
    Salami,
    Salmon,
    Sardine,
    Sauce,
    Sausage,
    Shrimp,
    Snail,
    Soup,
    Spinach,
    Steak,
    Strawberry,
    Sugar,
    Sushi,
    TamTim,
    Tofu,
    Tomato,
    Trout,
    Tuna,
    Vegemite,
    Walnut,
    Water,
    Watermelon,
    Worm,
    Yogurt,
}
