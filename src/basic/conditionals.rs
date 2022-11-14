#[allow(dead_code)]
pub fn examples_of_conditionals() {
    if 1 == 2 {
        println!("True, the numbers are equal."); //
    } else {
        println!("False, the numbers are not equal.");
    }

    println!("-----------------------------------------------------");

    let formal = true;
    let greeting = if formal {
        // if used here as an expression
        "Good day to you." // return a String
    } else {
        "Hey!"
    };
    println!("{}", greeting);

    println!("-----------------------------------------------------");
    let num = 500;
    let out_of_range: bool;
    if num < 0 {
        out_of_range = true;
    } else if num == 0 {
        out_of_range = true;
    } else if num > 512 {
        out_of_range = true;
    } else {
        out_of_range = false;
    }
    println!("out_of_range {}", out_of_range);
}
