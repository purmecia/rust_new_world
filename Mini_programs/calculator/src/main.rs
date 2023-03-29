use std::io;

fn main() {
    let mut input = String::new();

    // keep reading until 'q' to quit
    loop {
        println!("Enter an expression (e.g. 2 + 3, 'q' to quit):");

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let tokens: Vec<&str> = input.split_whitespace().collect();

        // if user enters 'q' to quit
        if tokens[0] == "q" {
            println!("Quitting...");
            break;
        }

        if tokens.len() != 3 {
            println!("tokens len: {}", tokens.len());
            println!("Invalid expression. Please enter in the format 'number operator number'");
            return;
        }

        let num1 = tokens[0].parse::<f64>().unwrap_or_else(|_| {
            println!("Invalid number: {}", tokens[0]);
            std::process::exit(1);
        });

        let num2 = tokens[2].parse::<f64>().unwrap_or_else(|_| {
            println!("Invalid number: {}", tokens[2]);
            std::process::exit(1);
        });

        let result = match tokens[1] {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Division by zero is not allowed");
                    std::process::exit(1);
                } else {
                    num1 / num2
                }
            }
            _ => {
                println!("Invalid operator: {}", tokens[1]);
                std::process::exit(1);
            }
        };

        println!("Result: {}", result);
    }
}