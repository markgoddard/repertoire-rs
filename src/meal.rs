use crate::food_groups::{Carbs, Protein};

#[derive(Clone, Debug)]
pub(crate) struct Meal {
    pub name: String,
    pub carbs: Carbs,
    pub protein: Protein,
}

impl Meal {
    pub(crate) fn new(name: &str, carbs: Carbs, protein: Protein) -> Self
    {
        Self { name: name.to_string(), carbs: carbs, protein: protein, }
    }
}