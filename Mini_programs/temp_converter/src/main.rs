use std::io;

fn main() {
    println!("Welcome to the Temperature Converter!");

    loop {
        println!("Please choose the conversion you would like to perform:");
        println!("1: Celsius to Fahrenheit");
        println!("2: Fahrenheit to Celsius");
        println!("3: Quit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 3.");
                continue;
            }
        };

        if choice == 3 {
            break;
        }

        println!("Enter the temperature to be converted:");
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read input");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid temperature.");
                continue;
            }
        };

        match choice {
            1 => {
                let result = celsius_to_fahrenheit(temp);
                println!("{:.2} Celsius is equal to {:.2} Fahrenheit", temp, result);
            }
            2 => {
                let result = fahrenheit_to_celsius(temp);
                println!("{:.2} Fahrenheit is equal to {:.2} Celsius", temp, result);
            }
            _ => {
                println!("Invalid choice. Please select a valid option.");
            }
        }
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}
    
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}