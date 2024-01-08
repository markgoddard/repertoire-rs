use crate::food_groups::{Carbs, Protein};

#[derive(Clone, Debug)]
pub(crate) struct Meal {
    pub name: String,
    pub carbs: Option<Carbs>,
    pub protein: Option<Protein>,
}

impl Meal {
    pub(crate) fn new(name: &str, carbs: Option<Carbs>, protein: Option<Protein>) -> Self
    {
        Self { name: name.to_string(), carbs: carbs, protein: protein, }
    }
}