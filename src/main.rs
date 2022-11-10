mod basic;

use basic::collections::collections_examples;
use basic::variables::examples_of_variables;

#[allow(dead_code)]
fn main() {
    examples_of_variables();
    collections_examples();
    println!("Hello, world!");
}
