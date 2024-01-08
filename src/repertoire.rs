use crate::meal_option::MealOption;

#[derive(Clone, Debug)]
pub(crate) struct Repertoire {
    pub(crate) meals: Vec<MealOption>,
}

impl Repertoire {
    pub(crate) fn new(meals: &[MealOption]) -> Self {
        Repertoire { meals: meals.to_vec() }
    }
}