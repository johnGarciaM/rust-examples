#[derive(PartialEq, Eq, Debug)]
// Declare Car struct to describe vehicle with four named fields
pub struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug)]
pub enum Age {
    New,
    Used,
}

fn car_quality(miles: u32) -> (Age, u32) {
    let quality: (Age, u32) = if miles == 0 {
        (Age::New, miles)
    } else {
        (Age::Used, miles)
    };
    quality
}

#[allow(dead_code)]
pub fn car_factory(order: i32, miles: u32) -> Car {
    let colors: [&str; 4] = ["Blue", "Green", "Red", "Silver"];

    let mut color = order as usize;

    while color > 4 {
        color -= 4
    }

    let mut motor = Transmission::Manual;
    let mut roof = true;

    if order % 3 == 0 {
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {
        motor = Transmission::SemiAuto;
        roof = false;
    }

    Car {
        color: String::from(colors[(color - 1) as usize]),
        motor,
        roof,
        age: car_quality(miles),
    }
}

#[allow(dead_code)]
pub fn build_car() {
    use std::collections::HashMap;
    let mut miles: u32 = 0;
    // let mut order: i32 = 1;
    let mut orders: HashMap<i32, Car> = HashMap::new();
    let mut car: Car;

    for order in 1..12 {
        car = car_factory(order, miles);
        orders.insert(order, car);
        println!("Car order {}: {:?}", order, orders.get(&order));

        // Reset miles for order variety
        if miles == 2100 {
            miles = 0;
        } else {
            miles += 700;
        }
    }
}
