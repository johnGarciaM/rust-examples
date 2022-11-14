#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
pub struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
// Declare enum for Car transmission type
pub enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum Age {
    New,
    Used,
}

#[allow(dead_code)]
fn car_quality(miles: u32) -> (Age, u32) {
    let quality: (Age, u32) = if miles == 0 {
        (Age::New, miles)
    } else {
        (Age::Used, miles)
    };
    quality
}

#[allow(dead_code)]
pub fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    let age: (Age, u32) = car_quality(miles);
    if age.0 == Age::Used {
        if roof {
            println!(
                "Prepare a used car: {:?}, {}, Hard top, {} miles\n",
                motor, color, miles
            );
        } else {
            println!(
                "Preparing a used car: {:?}, {}, Convertible, {} miles",
                motor, color, miles
            );
        }
    } else {
        if roof {
            println!(
                "Build a new car: {:?}, {}, Hard top, {} miles\n",
                motor, color, miles
            );
        } else {
            println!(
                "Building a new car: {:?}, {}, Convertible, {} miles",
                motor, color, miles
            );
        }
    }

    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: age,
    }
}

#[allow(dead_code)]
pub fn build_car() -> Car {
    #[allow(dead_code)]
    // Create car color array
    let colors: [&str; 4] = ["azul", "verde", "rojo", "plateado"];
    let mut engine: Transmission = Transmission::Manual;
    let mut car: Car;
    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[1]), engine, true, 0);
    println!(
        "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #2: Used, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[2]), engine, false, 100);
    println!(
        "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #3: Used, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[3]), engine, true, 200);
    println!(
        "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );
    car
}
