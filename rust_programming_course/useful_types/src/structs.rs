struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

pub fn examples() {
    let mut my_car = Car {
        owner: String::from("John"),
        year: 2020,
        fuel_level: 0.0,
        price: 1000,
    };

    let _ = my_car.year;
    my_car.fuel_level = 30.0;

    let _ = my_car.owner.clone();
    my_car.owner;

    let _ = Car {
        owner: String::from("Jane"),
        ..my_car
    };

    let _ = (1, 3);
    let _ = (4, 10, 13);

    // define struct for tuple
    struct Point2D(i32, i32);
    struct Point3D(i32, i32, i32);
}
