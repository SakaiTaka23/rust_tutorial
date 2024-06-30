enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

enum TravelType {
    Car(f32),
    Train,
    Airplane,
}

impl TravelType {
    fn travel_allowance(&self, miles: f32) -> f32 {
        match self {
            TravelType::Car(driven) => driven * 2.0,
            TravelType::Train => miles * 3.0,
            TravelType::Airplane => miles * 5.0,
        }
    }
}

pub fn examples() {
    let _ = WeekDay::Friday;
    let participant = TravelType::Car(100.0);
    println!("Allowance: {}", participant.travel_allowance(0.00));
    let participant = TravelType::Train;
    println!("Allowance: {}", participant.travel_allowance(100.0));
    let participant = TravelType::Airplane;
    println!("Allowance: {}", participant.travel_allowance(100.0));
}
