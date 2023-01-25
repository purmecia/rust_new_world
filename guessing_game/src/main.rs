use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Let's guess a number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");
    let mut correct: bool = false;

    for i in 0..10 {
        print!("\nYou have {} chance", 10 - i);
        if i < 9 {
            println!("s");
        }

        println!("Input the number you guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                correct = true;
                break;
            }
        }
    }
    if !correct {
        println!("You lose!");
        println!("The secret number is: {secret_number}");
    } else {
        println!("You win!");
    }
}
