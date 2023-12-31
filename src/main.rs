enum Animal {
    Dog(String),
    Cat(String),
    Bird(String),
}

fn main() {
    let my_pet = Animal::Cat("Fluffy".to_string());

    if let Animal::Cat(name) = my_pet {
        println!("I have a cat named {}", name);
    } else {
        println!("I don't have a cat");
    }
}
