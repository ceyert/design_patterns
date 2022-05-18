pub trait Packing {
    fn pack(&self) -> &'static str;
}

pub struct Wrapper;

pub struct Bottle;

impl Packing for Wrapper {
    fn pack(&self) -> &'static str {
        "Wrapper"
    }
}


impl Packing for Bottle {
    fn pack(&self) -> &'static str {
        "Bottle"
    }
}

pub trait Item {
    fn packing(&self) -> Box<dyn Packing>;
    fn get_name(&self) -> &'static str;
    fn get_price(&self) -> i32;
}

pub struct VegBurger;

pub struct ChickenBurger;

impl Item for VegBurger {
    fn packing(&self) -> Box<dyn Packing> {
        Box::new(Wrapper {})
    }

    fn get_name(&self) -> &'static str {
        "Veg Burger"
    }

    fn get_price(&self) -> i32 {
        25
    }
}


impl Item for ChickenBurger {
    fn packing(&self) -> Box<dyn Packing> {
        Box::new(Wrapper {})
    }
    fn get_name(&self) -> &'static str {
        "Chicken Burger"
    }

    fn get_price(&self) -> i32 {
        50
    }
}
pub struct Coke;

pub struct Pepsi;

impl Item for Coke {
    fn packing(&self) -> Box<dyn Packing> {
        Box::new(Bottle {})
    }
    fn get_name(&self) -> &'static str {
        "Coke"
    }

    fn get_price(&self) -> i32 {
        30
    }
}

impl Item for Pepsi {
    fn packing(&self) -> Box<dyn Packing> {
        Box::new(Bottle {})
    }
    fn get_name(&self) -> &'static str {
        "Pepsi"
    }

    fn get_price(&self) -> i32 {
        35
    }
}
pub struct Meal {
    pub items: Vec<Box<dyn Item>>
}

impl Meal {
    pub fn nw_instance() -> Meal {
        Meal { items: Vec::new() }
    }
}


pub struct MealBuilder;

pub enum DrinkType { COKE, PEPSI }

impl MealBuilder {
   pub fn prepare_veg_meal(drink_type: &DrinkType) -> Box<Meal> {
        let mut meal = Meal::nw_instance();
        meal.items.push(Box::new(VegBurger {}));
        match drink_type {
            DrinkType::COKE => meal.items.push(Box::new(Coke {})),
            DrinkType::PEPSI => meal.items.push(Box::new(Pepsi {})),
        }
        Box::new(meal)
    }

    pub fn prepare_non_veg_meal(drink_type: &DrinkType) -> Box<Meal> {
        let mut meal = Meal::nw_instance();
        meal.items.push(Box::new(ChickenBurger {}));
        match drink_type {
            DrinkType::COKE => meal.items.push(Box::new(Coke {})),
            DrinkType::PEPSI => meal.items.push(Box::new(Pepsi {})),
        }
        Box::new(meal)
    }
}



