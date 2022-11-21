#[allow(dead_code)]
pub fn examples_of_conditionals() {
    if 1 == 2 {
        println!("True, the numbers are equal."); //
    } else {
        println!("False, the numbers are not equal.");
    }

    println!("-----------------------------------------------------");

    let formal: bool = true;
    let greeting: &str = if formal {
        // if used here as an expression
        "Good day to you." // return a String
    } else {
        "Hey!"
    };
    println!("{}", greeting);

    println!("-----------------------------------------------------");
    let num: i32 = 500;
    // trunk-ignore(clippy/needless_late_init)
    let out_of_range: bool;
    if num < 0 && num == 0 && num > 512 {
        out_of_range = true;
    } else {
        out_of_range = false;
    }
    println!("out_of_range {}", out_of_range);
}
