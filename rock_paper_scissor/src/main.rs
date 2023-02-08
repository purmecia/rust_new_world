use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Let's play a game of rock, paper, scissors!");

    let computer_choice = rand::thread_rng().gen_range(1..4);

    println!("Input your choice (1 = rock, 2 = paper, 3 = scissors):");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //convert guess to a number
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    //print my choice
    match guess {
        1 => println!("You chose rock."),
        2 => println!("You chose paper."),
        3 => println!("You chose scissors."),
        _ => println!("You didn't choose a valid option."),
    }
    //print computer's choice
    match computer_choice {
        1 => println!("The computer chose rock."),
        2 => println!("The computer chose paper."),
        3 => println!("The computer chose scissors."),
        _ => println!("The computer didn't choose a valid option."),
    }

    //compare my guess to the computer's choice
    match guess.cmp(&computer_choice) {
        Ordering::Less => println!("You lose!"),
        Ordering::Greater => println!("You win!"),
        Ordering::Equal => println!("It's a tie!"),
    }

}
