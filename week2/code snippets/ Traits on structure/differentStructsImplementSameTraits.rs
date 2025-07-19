// Define a trait with 2 functions
trait Sound {
    fn engine_sound(&self) -> String;
    fn horn_sound(&self) -> String;
}


// Define the Car structure
struct Car {
    wheels_number: u8,
    speed: f64,
    color: String
}

impl Car {
    fn describe(&self) -> String {
        String::from(format!("My car has {} wheels, is {} and its speed is {}", self.wheels_number, self.color, self.speed))
    }
}
// Implement Sound trait for Car
impl Sound for Car {
    fn engine_sound(&self) -> String {
        String::from("Vroom")
    }


    fn horn_sound(&self) -> String {
        String::from("beep")
    }
}

struct Truck {
    wheels_number: u8,
    speed: f64,
    color: String,
    weight: u32,
}

impl Truck {
    fn describe(&self) -> String {
        String::from(format!("My truck has {} wheels, is {} and its speed is {}. It weights {}.", self.wheels_number, self.color, self.speed, self.weight))
    }
}
// Implement Sound trait for Truck
impl Sound for Truck {
    fn engine_sound(&self) -> String {
        String::from("VroomVroom")
    }


    fn horn_sound(&self) -> String {
        String::from("HONK!!!")
    }
}


// Main function
fn main() {
    
    let car = Car {
        wheels_number: 4,
        speed: 123.0,
        color: String::from("Blue"),
    };

    let truck = Truck {
        wheels_number: 4,
        speed: 123.0,
        color: String::from("Blue"),
        weight: 12350,
    };

    println!("Car description: {}", car.describe());
    println!("My car horn sound is {} and not {}!\n", car.horn_sound(), car.engine_sound());

    println!("Truck description: {}", truck.describe());
    println!("My truck horn sound is {} and not {}!", truck.horn_sound(), truck.engine_sound());

}