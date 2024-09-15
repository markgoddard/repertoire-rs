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
    let carbonara = MealOption::new("Carbonara", &[Carbs::Pasta], &[Protein::Pork]);
    let lasagne = MealOption::new("Lasagne", &[Carbs::Pasta], &[Protein::Beef]);
    let mut repertoire = Repertoire::new(&[carbonara, lasagne]);
    let mut plan = Plan::new();
    println!("Repertoire: {:?}", repertoire);
    while !repertoire.is_empty() {
        let meal_option = repertoire.random();
        let meal = repertoire.select(&meal_option, meal_option.carbs[0].clone(), meal_option.protein[0].clone());
        plan.add(meal);
    }
    println!("Plan: {:?}", plan);
}
