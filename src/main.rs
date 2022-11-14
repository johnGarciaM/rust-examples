mod basic;
mod exercises;

use basic::collections::collections_examples;
use basic::conditionals::examples_of_conditionals;
use basic::hash_maps::examples_hash_maps;
use basic::variables::examples_of_variables;
use exercises::exercise_01::build_car;

#[allow(dead_code)]
fn main() {
    // !todo explain declare variables
    examples_of_variables();
    println!("---------------------------------------------------------------");
    // !todo explain concepts
    collections_examples();
    println!("---------------------------------------------------------------");
    // !todo exercise-01
    build_car();
    println!("---------------------------------------------------------------");
    examples_of_conditionals();
    println!("---------------------------------------------------------------");
    examples_hash_maps();
    println!("---------------------------------------------------------------");
}
