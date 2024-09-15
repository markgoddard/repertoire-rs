use crate::food_groups::{Carbs, Protein};
use crate::meal_option::MealOption;
use crate::meal::Meal;

#[derive(Clone, Debug)]
pub(crate) struct Repertoire {
    pub(crate) meals: Vec<MealOption>,
}

impl Repertoire {
    pub(crate) fn new(meals: &[MealOption]) -> Self {
        Repertoire { meals: meals.to_vec() }
    }

    pub(crate) fn random(&self) -> MealOption {
        self.meals[0].clone()
    }

    pub(crate) fn select(&mut self, meal: &MealOption, carbs: Carbs, protein: Protein) -> Meal {
        let pos = self.meals.iter().position(|m| m == meal).unwrap();
        self.meals.remove(pos);
        meal.select(carbs, protein)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.meals.is_empty()
    }
}
