use crate::food_groups::{Carbs, Protein};
use crate::meal::Meal;

#[derive(Clone, Debug)]
pub(crate) struct MealOption {
    pub name: String,
    pub carbs: Vec<Carbs>,
    pub protein: Vec<Protein>,
}

impl MealOption {
    pub(crate) fn new(name: &str, carbs: &[Carbs], protein: &[Protein]) -> Self
    {
        Self { name: name.to_string(), carbs: carbs.to_vec(), protein: protein.to_vec(), }
    }

    pub(crate) fn select(&self, carbs: Carbs, protein: Protein) -> Meal {
        Meal::new(self.name.as_str(), carbs, protein)
    }
}