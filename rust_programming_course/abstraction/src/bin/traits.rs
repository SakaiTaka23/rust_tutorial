struct drawing_info {
    line_width: u8,
    color: String,
}

trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        0.0
    }
    fn status(&self) -> String {
        String::from("none")
    }
}

struct Square {
    side: f32,
    info: drawing_info,
}

impl Shape for Square {
    fn area(&self) -> f32 {
        self.side * self.side
    }

    fn status(&self) -> String {
        self.info.color.clone()
    }
}

struct Rectangle {
    length: f32,
    width: f32,
    info: drawing_info,
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.length * self.width
    }

    fn perimeter(&self) -> f32 {
        5.0
    }

    fn status(&self) -> String {
        self.info.color.clone()
    }
}

fn main() {
    let r = Rectangle {
        length: 5.0,
        width: 7.0,
        info: drawing_info {
            line_width: 3,
            color: String::from("Red"),
        },
    };
    println!("area is: {}", r.area());
}
