#[allow(dead_code)]
pub fn loops_examples() {
    // trunk-ignore(clippy/never_loop)
    loop {
        // Keep printing, printing, printing...
        println!("We loop forever!");
        // On the other hand, maybe we should stop!
        break;
    }

    let mut counter: i32 = 1;
    // stop_loop is set when loop stops
    let stop_loop: i32 = loop {
        counter *= 2;
        if counter > 100 {
            // Stop loop, return counter value
            break counter;
        }
    };
    // Loop should break when counter = 128
    println!("Break the loop at counter = {}.", stop_loop);

    counter = 4;

    while counter < 5 {
        println!("We loop a while...");
        counter += 1;
    }

    let big_birds: [&str; 3] = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
    }

    for number in 0..5 {
        println!("{}", number * 2);
    }
}
