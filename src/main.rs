mod food_groups;
mod meal;
mod meal_option;
mod plan;
mod repertoire;

use crate::food_groups::{Carbs, Protein};
use crate::meal_option::MealOption;
use crate::plan::Plan;
use crate::repertoire::Repertoire;

fn main() {
    let meal = MealOption::new("Carbonara", Some(&[Carbs::Pasta]), Some(&[Protein::Pork]));
    let repertoire = Repertoire::new(&[meal]);
    let mut plan = Plan::new();
    for meal in repertoire.meals.iter() {
        plan.add(meal.as_meal(Some(Carbs::Pasta), Some(Protein::Pork)))
    }
    println!("Repertoire: {:?}", repertoire);
    println!("Plan: {:?}", plan);
}