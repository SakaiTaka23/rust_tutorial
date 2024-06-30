use graphics::calculate_area;
use graphics::show_area;

mod graphics {
    pub use display::show_area;
    pub use shapes::calculate_area;

    pub mod shapes {
        pub fn calculate_area(radius: f64) -> f64 {
            std::f64::consts::PI * radius * radius
        }
    }
    pub mod display {
        pub fn show_area(shape: &str, area: f64) {
            println!("The area of the {} is: {}", shape, area);
        }
    }
}

// fix this line
fn main() {
    let radius = 3.0;
    let area = calculate_area(radius);

    show_area("circle", area);
}
