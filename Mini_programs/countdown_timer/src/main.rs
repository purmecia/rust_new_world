use std::io;
use std::time::Duration;
use std::thread;

fn main() {
    let mut input = String::new();
    println!("Enter the duration for the countdown in seconds:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let seconds = match input.trim().parse::<u64>() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    start_countdown(seconds);
}

fn start_countdown(seconds: u64) {
    for i in (1..=seconds).rev() {
        println!("{} seconds remaining...", i);
        thread::sleep(Duration::from_secs(1));
    }
    println!("Time's up!");
}
