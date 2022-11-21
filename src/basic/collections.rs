pub fn collections_examples() {
    // Declare array, initialize all values, compiler infers length = 7
    #[allow(dead_code)]
    let days: [&str; 7] = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    // Declare array, initialize all values to 0, length = 5
    #[allow(dead_code)]
    let bytes: [i32; 5] = [0; 5];

    println!("days {:?}, bytes {:?}", days, bytes);

    // Declare vector, initialize with three values
    let three_nums: Vec<i32> = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);

    // Declare vector, value = "0", length = 5
    let zeroes: Vec<i32> = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    // Create empty vector, declare vector mutable so it can grow and shrink
    let mut fruit: Vec<&str> = Vec::new();
    // Push values onto end of vector, type changes from generic `T` to String
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);

    // Pop off value at end of vector
    // Call pop() method from inside println! macro
    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);

    // Declare vector, initialize with three values
    let mut index_vec: Vec<i32> = vec![15, 3, 46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);

    // Add 5 to the value at index 1, which is 5 + 3 = 8
    index_vec[1] += 5;
    println!("Vector: {:?}", index_vec);

    // Access vector with out-of-bounds index
    let beyond: i32 = index_vec[2];
    println!("{}", beyond);
}
