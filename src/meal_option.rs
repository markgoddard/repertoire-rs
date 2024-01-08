use crate::food_groups::{Carbs, Protein};
use crate::meal::Meal;

#[derive(Clone, Debug)]
pub(crate) struct MealOption {
    pub name: String,
    pub carbs: Option<Vec<Carbs>>,
    pub protein: Option<Vec<Protein>>,
}

impl MealOption {
    pub(crate) fn new(name: &str, carbs: Option<&[Carbs]>, protein: Option<&[Protein]>) -> Self
    {
        Self { name: name.to_string(), carbs: carbs.map(|c| c.to_vec()), protein: protein.map(|p| p.to_vec()), }
    }

    pub(crate) fn as_meal(&self, carbs: Option<Carbs>, protein: Option<Protein>) -> Meal {
        // TODO: Validate carbs and protein
        Meal::new("foo", carbs, protein)
    }
}