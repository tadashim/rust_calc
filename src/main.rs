struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let my_rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };

    let area = my_rectangle.area();
    println!("Area: {}", area);
}
