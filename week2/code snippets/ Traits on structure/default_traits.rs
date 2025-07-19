// Define the Vehicle structure
struct Vehicle {
    wheels_number: u8,
    speed: f64,
    color: String
}

impl Vehicle {
    fn horn_sound(&self) -> String {
        String::from("tututtt")
    }

    fn engine_sound(&self) -> String {
        String::from("vroum")
    }

    fn describe(&self) -> String {
        String::from(format!("My car has {} wheels, is {} and its speed is {}", self.wheels_number, self.color, self.speed))
    }
}


// Main function
fn main() {
    
    let car = Vehicle {
        wheels_number: 4,
        speed: 123.0,
        color: String::from("Blue"),
    };

    println!("Car description: {}", car.describe());
    println!("My car horn sound is {} and not {}!", car.horn_sound(), car.engine_sound());
}