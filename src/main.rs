mod basic;
mod exercises;

use basic::collections::collections_examples;
use basic::variables::examples_of_variables;
use exercises::exercise_01::build_car;

#[allow(dead_code)]
fn main() {
    // !todo explain declare variables
    examples_of_variables();
    // !todo explain concepts
    collections_examples();
    // !todo exercise-01
    build_car();
}
