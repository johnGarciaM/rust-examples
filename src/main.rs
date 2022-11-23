mod basic;
mod exercises;

use basic::collections::collections_examples;
use basic::conditionals::conditionals_examples;
use basic::hash_maps::hash_maps_examples;
use basic::loops::loops_examples;
use basic::variables::variables_examples;
use exercises::exercise_01::build_car;

#[allow(dead_code)]
fn main() {
    // !todo explain declare variables
    variables_examples();
    println!("---------------------------------------------------------------");
    // !todo explain concepts
    collections_examples();
    println!("---------------------------------------------------------------");
    conditionals_examples();
    println!("---------------------------------------------------------------");
    hash_maps_examples();
    println!("---------------------------------------------------------------");
    loops_examples();
    println!("---------------------------------------------------------------");
    // !todo exercise-01
    build_car();
    println!("---------------------------------------------------------------");
}
