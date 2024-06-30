struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

impl Car {
    fn monthly_insurance() -> u32 {
        123
    }

    fn selling_price(&self) -> u32 {
        self.price + Car::monthly_insurance()
    }

    fn new(name: String, year: u32) -> Self {
        Car {
            owner: name,
            year,
            fuel_level: 0.0,
            price: 0,
        }
    }

    fn display_car_info(&self) {
        println!(
            "owner: {}, year: {}, fuel_level: {}, price: {}",
            self.owner,
            self.year,
            self.fuel_level,
            self.price
        );
    }

    fn refuel(&mut self, gallons: f32) {
        self.fuel_level += gallons;
    }

    fn sell(self, owner_name: String) -> Self {
        Car {
            owner: owner_name,
            ..self
        }
    }
}

pub fn examples() {
    let mut my_car = Car {
        owner: String::from("John"),
        year: 2022,
        fuel_level: 0.0,
        price: 10000,
    };

    my_car.display_car_info();
    my_car.refuel(10.5);

    let new_owner = my_car.sell(String::from("Jane"));
    new_owner.display_car_info();
    let _ = new_owner.selling_price();

    let _ = Car::new(String::from("Home"), 2022);
}
