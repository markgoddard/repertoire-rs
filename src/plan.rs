use crate::meal::Meal;

#[derive(Clone, Debug)]
pub(crate) struct Plan {
    // Could become a map of day -> meal.
    meals: Vec<Meal>,
}

impl Plan {
    pub(crate) fn new() -> Plan {
        Plan { meals: Vec::new() }
    }

    pub(crate) fn add(&mut self, meal: Meal) {
        self.meals.push(meal)
    }
}