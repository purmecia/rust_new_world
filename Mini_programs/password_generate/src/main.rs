use rand::Rng;

fn main() {
    // Define the character set from which to generate the password.
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()-_=+[{]}\\|;:'\",<.>/?";
    let char_vec: Vec<char> = chars.chars().collect();
    
    // Get the desired length of the password from the user.
    println!("How long should the password be?");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let password_length = input.trim().parse::<usize>().unwrap();
    
    // Generate the password.
    let password: String = (0..password_length)
        .map(|_| char_vec[rand::thread_rng().gen_range(0..char_vec.len())])
        .collect();
    
    // Print the password to the console.
    println!("Password: {}", password);
}